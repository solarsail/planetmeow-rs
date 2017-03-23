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
    pub deleted: bool,
}


use super::schema::posts;

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub category: String,
    pub body: String,
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct Visitor {
    pub id: i32,
    pub name: String,
    pub mail: String,
    pub site: Option<String>,
    pub created: NaiveDateTime,
}


use super::schema::visitors;

#[derive(Insertable, Deserialize)]
#[table_name="visitors"]
pub struct NewVisitor {
    pub name: String,
    pub mail: String,
    pub site: Option<String>,
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub pid: i32,
    pub vid: i32,
    pub body: String,
    pub created: NaiveDateTime,
    pub last_edited: NaiveDateTime,
    pub deleted: bool,
}


use super::schema::comments;

#[derive(Insertable, Deserialize)]
#[table_name="comments"]
pub struct NewComment {
    pub pid: i32,
    pub vid: i32,
    pub body: String,
}
