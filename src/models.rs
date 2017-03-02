use diesel::data_types::*;


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub body: String,
    pub created: PgTimestamp,
    pub last_edited: PgTimestamp,
    pub published: bool,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub category: String,
    pub body: String,
}
