use rocket::Request;
use rocket_contrib::{ JSON, Value };
use super::super::models::Post;
use super::super::db;



#[get("/post")]
pub fn get_all(db: db::DB) -> JSON<Vec<Post>> {
    JSON(db::get_published_posts(db.conn()))
}


#[get("/post/<id>")]
pub fn get(db: db::DB, id: i32) -> Option<JSON<Post>> {
    let post = db::get_published_post(db.conn(), id);
    match post {
        Ok(p) => Some(JSON(p)),
        _ => None
    }
}


#[derive(Serialize, Deserialize)]
pub struct PostInput {
    title: String,
    categories: Vec<String>,
    body: String,
}

#[post("/post/create", format="application/json", data="<post>")]
pub fn create(db: db::DB, post: JSON<PostInput>) -> JSON<Value> { // returns id
    let cats = if post.categories.len() > 0 {
        Some(&post.categories)
    } else {
        None
    };
    let post = db::create_post(db.conn(), &post.title, cats, &post.body);
    match post {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        _ => JSON(json!({ "status": "database error" }))
    }
}


#[post("/post/<id>", format="application/json", data="<post>")]
pub fn update(db: db::DB, id: i32, post: JSON<PostInput>) -> JSON<Value> { // returns id
    let cats = if post.categories.len() > 0 {
        Some(&post.categories)
    } else {
        None
    };
    let post = db::update_post(db.conn(), id, &post.title, cats, &post.body);
    match post {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        Err(db::Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[post("/post/<id>/publish")]
pub fn publish(db: db::DB, id: i32) -> JSON<Value> {
    let post = db::publish_post(db.conn(), id);
    match post {
        Ok(p) => JSON(json!({ "status": "ok", "id": id })),
        Err(db::Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[delete("/post/<id>")]
pub fn delete(db: db::DB, id: i32) -> JSON<Value> {
    let num = db::delete_post(db.conn(), id);
    match num {
        Ok(n) => JSON(json!({ "status": "ok", "id": id })),
        Err(db::Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}
