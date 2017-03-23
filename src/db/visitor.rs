// DB ORM
use diesel;
use diesel::prelude::*;
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

