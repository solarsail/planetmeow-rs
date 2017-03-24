// DB ORM
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::pg::PgConnection;

use db::{Error, DBResult};
use models::{Visitor, NewVisitor};

pub fn create(conn: &PgConnection, name: &str, mail: &str, site: Option<String>) -> DBResult<Visitor> {
    use schema::visitors;

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


pub fn get(conn:&PgConnection, id: Option<i32>) -> Vec<Visitor> {
    use schema::visitors;

    let mut query = visitors::table.into_boxed();
    if let Some(vid) = id {
        query = query.filter(visitors::id.eq(vid));
    }

    let ret = query.load::<Visitor>(conn);
    match ret {
        Ok(v) => v,
        _ => Vec::new()
    }
}


pub fn update(conn: &PgConnection, id: i32, name: &str, mail: &str, site: Option<String>) -> DBResult<Visitor> {
    use schema::visitors;

    diesel::update(visitors::table.find(id))
            .set((visitors::name.eq(name),
                  visitors::mail.eq(mail),
                  visitors::site.eq(site),
                  ))
            .get_result(conn)
            .map(|v| v)
            .map_err(|e| match e {
                DieselError::NotFound => Error::RecordNotFound,
                _ => Error::DatabaseError
            })
}


pub fn delete(conn: &PgConnection, id: i32) -> DBResult<usize> {
    use schema::visitors;

    diesel::delete(visitors::table.find(id))
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
    fn test_visitor() {
        use db::DB_POOL;
        let ref conn = DB_POOL.get().unwrap();

        // Create
        let name = "visitor1";
        let mail = "test@test.com";
        let site = Some("a".into());

        let visitor = create(conn, name, mail, site.clone()).unwrap();
        assert!(visitor.name == name && visitor.mail == mail
                && visitor.site == site);

        let visitor_id = visitor.id;

        // Retrieve
        let ref visitor = get(conn, Some(visitor_id))[0];
        assert!(visitor.name == name && visitor.mail == mail
                && visitor.site == site);

        // Update
        let name = "visitor2";
        let mail = "test2@test.com";
        let site = None;
        let visitor = update(conn, visitor_id, name, mail, site).unwrap();
        assert!(visitor.name == name && visitor.mail == mail
                && visitor.site == None);

        // Delete
        let num = delete(conn, visitor_id).unwrap();
        assert!(num == 1);

    }
}

