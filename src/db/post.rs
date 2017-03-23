// DB ORM
use diesel;
use diesel::prelude::*;
use diesel::data_types::PgTimestamp;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;

// Timestamp
use chrono::prelude::*;

use models::{Post, NewPost};
use db::{Error, DBResult};


fn serialize_categories(cats: Option<&Vec<String>>) -> String {
    cats.map_or("".into(), |v| v.join(","))
}


pub fn create(conn: &PgConnection,
                       title: &str, categories: Option<&Vec<String>>, body: &str) -> DBResult<Post> {
    use schema::posts;

    let new_post = NewPost {
        title: title.into(),
        category: serialize_categories(categories),
        body: body.into(),
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .map(|post| post)
        .map_err(|_| Error::DatabaseError)
}


pub fn update(conn: &PgConnection,
                       id: i32, title: &str, categories: Option<&Vec<String>>, body: &str) -> DBResult<Post> {
    use schema::posts;

    let cat = serialize_categories(categories);
    let millennium= NaiveDateTime::from_timestamp(946684800, 0);
    let now = UTC::now().naive_utc();
    let ts = now.signed_duration_since(millennium).num_microseconds().unwrap();
    diesel::update(posts::table.find(id))
        .set((
                posts::title.eq(title),
                posts::category.eq(cat),
                posts::body.eq(body),
                posts::last_edited.eq(PgTimestamp(ts))
             ))
        .get_result(conn)
        .map(|post| post)
        .map_err(|e| match e {
            DieselError::NotFound => Error::RecordNotFound,
            _ => Error::DatabaseError
        })
}


pub fn get(conn: &PgConnection, id: Option<i32>, published_only: bool, non_deleted_only: bool) -> Vec<Post> {
    use schema::posts;

    let mut query = posts::table.into_boxed();
    if let Some(pid) = id {
        query = query.filter(posts::id.eq(pid));
    }
    if published_only {
        query = query.filter(posts::published.eq(true));
    }
    if non_deleted_only {
        query = query.filter(posts::deleted.eq(false));
    }

    let ret = query.load::<Post>(conn);
    match ret {
        Ok(v) => v,
        _ => Vec::new()
    }
}

pub fn get_published(conn: &PgConnection, id: Option<i32>) -> Vec<Post> {
    get(conn, id, true, true)
}


pub fn get_all(conn: &PgConnection) -> Vec<Post> {
    get(conn, None, false, false)
}


pub fn publish(conn: &PgConnection, id: i32) -> DBResult<Post> {
    use schema::posts::dsl;

    diesel::update(dsl::posts.find(id))
        .set(dsl::published.eq(true))
        .get_result(conn)
        .map(|post| post)
        .map_err(|e| match e {
            DieselError::NotFound => Error::RecordNotFound,
            _ => Error::DatabaseError
        })
}


pub fn delete(conn: &PgConnection, id: i32) -> DBResult<usize> {
    use schema::posts::dsl;

    diesel::update(dsl::posts.find(id))
            .set(dsl::deleted.eq(true))
            .execute(conn)
            .map(|num| num)
            .map_err(|e| match e {
                DieselError::NotFound => Error::RecordNotFound,
                _ => Error::DatabaseError
            })
}


pub fn purge(conn: &PgConnection) -> DBResult<usize> {
    use schema::posts::dsl;

    diesel::delete(dsl::posts.filter(dsl::deleted.eq(true)))
            .execute(conn)
            .map(|num| num)
            .map_err(|e| match e {
                DieselError::NotFound => Error::RecordNotFound, // FIXME: necessary?
                _ => Error::DatabaseError
            })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_post() {
        use db::DB_POOL;
        use chrono::Duration;

        let ref conn = DB_POOL.get().unwrap();
        // Create
        let title = "title1";
        let cats = vec!["tag1".into(), "tag2".into()];
        let body = "body1";

        let post = create(conn, title, Some(&cats), body).unwrap();
        assert!(post.title == title && post.category == "tag1,tag2"
                && post.body == body && post.published == false);
        assert!(post.created == post.last_edited);

        let post_id = post.id;

        // Retrieve draft
        let posts = get_published(conn, Some(post_id));
        assert!(posts.len() == 0);

        // Update
        let title = "title2";
        let body = "body2";

        let post = update(conn, post_id, title, None, body).unwrap();
        println!("created: {:?}, updated: {:?}", post.created, post.last_edited);
        assert!(post.title == title && post.category == ""
                && post.body == body && post.published == false);
        assert!(post.created < post.last_edited);
        assert!(post.last_edited.signed_duration_since(post.created) < Duration::milliseconds(500));

        // Publish
        let post = publish(conn, post_id).unwrap();
        assert!(post.published);

        // Retrieve published
        let ref post = get(conn, Some(post_id), false, false)[0];
        assert!(post.title == title && post.category == ""
                && post.body == body && post.published == true);

        // Delete
        let num = delete(conn, post.id).unwrap();
        assert!(num == 1);
        let num = purge(conn).unwrap();
        println!("purged: {}", num);
        assert!(num == 1);

        // Batch retrieve
        let pv1 = get_published(conn, None);
        let post1 = create(conn, "t1", Some(&cats), "b1").unwrap();
        let post2 = create(conn, "t2", None, "b2").unwrap();
        let pv2 = get_published(conn, None);
        assert!(pv2.len() == pv1.len());
        let post1 = publish(conn, post1.id).unwrap();
        let post2 = publish(conn, post2.id).unwrap();
        let pv2 = get_published(conn, None);
        assert!(pv2.len() == pv1.len() + 2);
        let num = delete(conn, post1.id).unwrap();
        assert!(num == 1);
        let num = delete(conn, post2.id).unwrap();
        assert!(num == 1);
        let num = purge(conn).unwrap();
        assert!(num == 2);
    }
}
