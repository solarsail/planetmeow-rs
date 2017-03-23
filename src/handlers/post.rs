use rocket_contrib::{ JSON, Value };
use models::Post;
use db::{DB, post, Error};



#[get("/post")]
pub fn get_all(db: DB) -> JSON<Vec<Post>> {
    JSON(post::get_published(db.conn(), None))
}


#[get("/post/<id>")]
pub fn get(db: DB, id: i32) -> Option<JSON<Post>> {
    let mut posts = post::get_published(db.conn(), Some(id));
    posts.pop().map(|p| JSON(p))
}


#[derive(Serialize, Deserialize)]
pub struct PostInput {
    title: String,
    categories: Vec<String>,
    body: String,
}

#[post("/post/create", format="application/json", data="<post>")]
pub fn create(db: DB, post: JSON<PostInput>) -> JSON<Value> { // returns id
    let cats = if post.categories.len() > 0 {
        Some(&post.categories)
    } else {
        None
    };
    let post = post::create(db.conn(), &post.title, cats, &post.body);
    match post {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        _ => JSON(json!({ "status": "database error" }))
    }
}


#[post("/post/<id>", format="application/json", data="<post>")]
pub fn update(db: DB, id: i32, post: JSON<PostInput>) -> JSON<Value> { // returns id
    let cats = if post.categories.len() > 0 {
        Some(&post.categories)
    } else {
        None
    };
    let post = post::update(db.conn(), id, &post.title, cats, &post.body);
    match post {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[post("/post/<id>/publish")]
pub fn publish(db: DB, id: i32) -> JSON<Value> {
    let post = post::publish(db.conn(), id);
    match post {
        Ok(_) => JSON(json!({ "status": "ok", "id": id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[delete("/post/<id>")]
pub fn delete(db: DB, id: i32) -> JSON<Value> {
    let num = post::delete(db.conn(), id);
    match num {
        Ok(_) => JSON(json!({ "status": "ok", "id": id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}
