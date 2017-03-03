use chrono::prelude::*;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub body: String,
    pub created: NaiveDateTime,
    pub last_edited: NaiveDateTime,
    pub published: bool,
}


use super::schema::posts;

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub category: String,
    pub body: String,
}
