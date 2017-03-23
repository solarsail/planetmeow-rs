// DB ORM
use diesel;
use diesel::prelude::*;
use diesel::data_types::PgTimestamp;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;

// Connection pool
use r2d2::{ Pool, Config, PooledConnection, GetTimeout };
use r2d2_diesel::ConnectionManager;

// Environment
use dotenv::dotenv;
use std::env;

// Timestamp
use chrono::prelude::*;
use chrono::Duration;

// Provides DB access for Rocket
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;
use rocket::Request;


use super::models::{ Post, NewPost, Visitor, NewVisitor, Comment, NewComment };


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RecordNotFound,
    DatabaseError,
}


pub type DBResult<T> = Result<T, Error>;


pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;

    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}


lazy_static! {
    static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}


fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}


fn serialize_categories(cats: Option<&Vec<String>>) -> String {
    cats.map_or("".into(), |v| v.join(","))
}


pub fn create_post(conn: &PgConnection,
                       title: &str, categories: Option<&Vec<String>>, body: &str) -> DBResult<Post> {
    use super::schema::posts;

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


pub fn update_post(conn: &PgConnection,
                       id: i32, title: &str, categories: Option<&Vec<String>>, body: &str) -> DBResult<Post> {
    use super::schema::posts::dsl;

    let cat = serialize_categories(categories);
    let millennium= NaiveDateTime::from_timestamp(946684800, 0);
    let now = UTC::now().naive_utc();
    let ts = now.signed_duration_since(millennium).num_microseconds().unwrap();
    diesel::update(dsl::posts.find(id))
        .set((
                dsl::title.eq(title),
                dsl::category.eq(cat),
                dsl::body.eq(body),
                dsl::last_edited.eq(PgTimestamp(ts))
             ))
        .get_result(conn)
        .map(|post| post)
        .map_err(|e| match e {
            DieselError::NotFound => Error::RecordNotFound,
            _ => Error::DatabaseError
        })
}


pub fn get_post(conn: &PgConnection, id: i32, published_only: bool, non_deleted_only: bool) -> DBResult<Post> {
    use super::schema::posts;

    let mut query = posts::table.into_boxed();
    query = query.filter(posts::id.eq(id));
    if published_only {
        query = query.filter(posts::published.eq(true));
    }
    if non_deleted_only {
        query = query.filter(posts::deleted.eq(false));
    }

    query.get_result(conn)
         .map(|post| post)
         .map_err(|e| match e {
             DieselError::NotFound => Error::RecordNotFound,
             _ => Error::DatabaseError
         })
}

pub fn get_published_post(conn: &PgConnection, id: i32) -> DBResult<Post> {
    get_post(conn, id, true, true)
}


pub fn get_published_posts(conn: &PgConnection) -> Vec<Post> {
    use super::schema::posts::dsl;

    let ret = dsl::posts
        .filter(dsl::published.eq(true))
        .filter(dsl::deleted.eq(false))
        .load::<Post>(conn);
    match ret {
        Ok(v) => v,
        _ => Vec::new()
    }
}


pub fn publish_post(conn: &PgConnection, id: i32) -> DBResult<Post> {
    use super::schema::posts::dsl;

    diesel::update(dsl::posts.find(id))
        .set(dsl::published.eq(true))
        .get_result(conn)
        .map(|post| post)
        .map_err(|e| match e {
            DieselError::NotFound => Error::RecordNotFound,
            _ => Error::DatabaseError
        })
}


pub fn delete_post(conn: &PgConnection, id: i32) -> DBResult<usize> {
    use super::schema::posts::dsl;

    diesel::update(dsl::posts.find(id))
            .set(dsl::deleted.eq(true))
            .execute(conn)
            .map(|num| num)
            .map_err(|e| match e {
                DieselError::NotFound => Error::RecordNotFound,
                _ => Error::DatabaseError
            })
}


pub fn purge_posts(conn: &PgConnection) -> DBResult<usize> {
    use super::schema::posts::dsl;

    diesel::delete(dsl::posts.filter(dsl::deleted.eq(true)))
            .execute(conn)
            .map(|num| num)
            .map_err(|e| match e {
                DieselError::NotFound => Error::RecordNotFound, // FIXME: necessary?
                _ => Error::DatabaseError
            })
}



pub fn create_visitor(conn: &PgConnection, name: &str, mail: &str, site: Option<String>) -> DBResult<Visitor> {
    use super::schema::visitors;

    let new_visitor = NewVisitor {
        name: name.into(),
        mail: mail.into(),
        site: site,
    };

    diesel::insert(&new_visitor).into(visitors::table)
        .get_result(conn)
        .map(|visitor| visitor)
        .map_err(|_| Error::DatabaseError)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crud() {
        let ref conn = DB_POOL.get().unwrap();
        // Create
        let title = "title1";
        let cats = vec!["tag1".into(), "tag2".into()];
        let body = "body1";

        let post = create_post(conn, title, Some(&cats), body).unwrap();
        assert!(post.title == title && post.category == "tag1,tag2"
                && post.body == body && post.published == false);
        assert!(post.created == post.last_edited);

        let post_id = post.id;

        // Retrieve draft
        let post = get_published_post(conn, post_id);
        assert!(match post {
            Err(Error::RecordNotFound) => true,
            _ => false
        });

        // Update
        let title = "title2";
        let body = "body2";

        let post = update_post(conn, post_id, title, None, body).unwrap();
        println!("created: {:?}, updated: {:?}", post.created, post.last_edited);
        assert!(post.title == title && post.category == ""
                && post.body == body && post.published == false);
        assert!(post.created < post.last_edited);
        assert!(post.last_edited.signed_duration_since(post.created) < Duration::milliseconds(500));

        // Publish
        let post = publish_post(conn, post_id).unwrap();
        assert!(post.published);

        // Retrieve published
        let post = get_post(conn, post_id, false, false).unwrap();
        assert!(post.title == title && post.category == ""
                && post.body == body && post.published == true);

        // Delete
        let num = delete_post(conn, post.id).unwrap();
        assert!(num == 1);
        let num = purge_posts(conn).unwrap();
        println!("purged: {}", num);
        assert!(num == 1);

        // Batch retrieve
        let pv1 = get_published_posts(conn);
        let post1 = create_post(conn, "t1", Some(&cats), "b1").unwrap();
        let post2 = create_post(conn, "t2", None, "b2").unwrap();
        let pv2 = get_published_posts(conn);
        assert!(pv2.len() == pv1.len());
        let post1 = publish_post(conn, post1.id).unwrap();
        let post2 = publish_post(conn, post2.id).unwrap();
        let pv2 = get_published_posts(conn);
        assert!(pv2.len() == pv1.len() + 2);
        let num = delete_post(conn, post1.id).unwrap();
        assert!(num == 1);
        let num = delete_post(conn, post2.id).unwrap();
        assert!(num == 1);
        let num = purge_posts(conn).unwrap();
        assert!(num == 2);
    }
}
