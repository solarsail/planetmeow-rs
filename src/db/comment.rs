// DB ORM
use diesel;
use diesel::prelude::*;
use diesel::data_types::PgTimestamp;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;

// Timestamp
use chrono::prelude::*;

use schema::comments;
use models::{Comment, NewComment};
use db::{Error, DBResult};


pub fn create(conn: &PgConnection, pid: i32, vid: i32, body: &str) -> DBResult<Comment> {
    let new_cmt = NewComment {
        pid: pid,
        vid: vid,
        body: body.into(),
    };

    diesel::insert(&new_cmt).into(comments::table)
        .get_result(conn)
        .map(|cmt| cmt)
        .map_err(|e| match e {
            DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => Error::ForeignKeyViolation,
            _ => Error::DatabaseError
        })
}


pub fn update(conn: &PgConnection, id: i32, body: &str) -> DBResult<Comment> {
    let millennium= NaiveDateTime::from_timestamp(946684800, 0);
    let now = UTC::now().naive_utc();
    let ts = now.signed_duration_since(millennium).num_microseconds().unwrap();
    diesel::update(comments::table.find(id))
        .set((
                comments::body.eq(body),
                comments::last_edited.eq(PgTimestamp(ts))
             ))
        .get_result(conn)
        .map(|post| post)
        .map_err(|e| match e {
            DieselError::NotFound => Error::RecordNotFound,
            DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => Error::ForeignKeyViolation,
            _ => Error::DatabaseError
        })
}


pub fn get(conn: &PgConnection, id: Option<i32>, non_deleted_only: bool) -> Vec<Comment> {
    let mut query = comments::table.into_boxed();
    if let Some(cid) = id {
        query = query.filter(comments::id.eq(cid));
    }
    if non_deleted_only {
        query = query.filter(comments::deleted.eq(false));
    }

    let ret = query.load::<Comment>(conn);
    match ret {
        Ok(v) => v,
        _ => Vec::new()
    }
}


pub fn delete(conn: &PgConnection, id: i32) -> DBResult<usize> {
    diesel::update(comments::table.find(id))
            .set(comments::deleted.eq(true))
            .execute(conn)
            .map(|num| num)
            .map_err(|e| match e {
                DieselError::NotFound => Error::RecordNotFound,
                _ => Error::DatabaseError
            })
}


pub fn purge(conn: &PgConnection) -> DBResult<usize> {
    diesel::delete(comments::table.filter(comments::deleted.eq(true)))
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
    use db::{post, visitor};

    #[test]
    fn test_comment() {
        use db::DB_POOL;
        use chrono::Duration;

        let ref conn = DB_POOL.get().unwrap();
        // Create
        let title = "title1";
        let cats = vec!["tag1".into(), "tag2".into()];
        let body = "body1";

        let post = post::create(conn, title, Some(&cats), body).unwrap();
        let post = post::publish(conn, post.id).unwrap();

        let body = "comment body";
        let visitor = visitor::create(conn, "visitor1", "test@test.com", None).unwrap();
        let comment = create(conn, post.id, visitor.id, body).unwrap();
        assert!(comment.body == body);
        assert!(comment.vid == visitor.id, "vid: {}, visitor id: {}", comment.vid, visitor.id);
        assert!(comment.pid == post.id, "pid: {}, post id: {}", comment.pid, post.id);

        // Delete
        let num = delete(conn, comment.id).unwrap();
        assert!(num == 1);
        let num = purge(conn).unwrap();
        println!("purged: {}", num);
        assert!(num == 1);

        visitor::delete(conn, visitor.id).unwrap();
        post::delete(conn, post.id).unwrap();
        post::purge(conn).unwrap();
    }
}
