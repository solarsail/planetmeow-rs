use rocket_contrib::{JSON, Value};
use models::{Comment, NewComment};
use db::{DB, comment, Error};



#[get("/comment")]
pub fn get_all(db: DB) -> JSON<Vec<Comment>> {
    JSON(comment::get(db.conn(), None, false))
}


#[get("/comment/<id>")]
pub fn get(db: DB, id: i32) -> Option<JSON<Comment>> {
    let mut comment = comment::get(db.conn(), Some(id), true);
    comment.pop().map(|v| JSON(v))
}


#[post("/comment/create", format="application/json", data="<comment>")]
pub fn create(db: DB, comment: JSON<NewComment>) -> JSON<Value> { // returns id
    let comment = comment::create(db.conn(), comment.pid, comment.vid, &comment.body);
    match comment {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        Err(Error::ForeignKeyViolation) =>
            JSON(json!({ "status": "error", "description": "Invalid post id or visitor id" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[post("/comment/<id>", format="application/json", data="<comment>")]
pub fn update(db: DB, id: i32, comment: JSON<NewComment>) -> JSON<Value> { // returns id
    let comment = comment::update(db.conn(), id, &comment.body);
    match comment {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        Err(Error::ForeignKeyViolation) =>
            JSON(json!({ "status": "error", "description": "Invalid post id or visitor id" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[delete("/comment/<id>")]
pub fn delete(db: DB, id: i32) -> JSON<Value> {
    let num = comment::delete(db.conn(), id);
    match num {
        Ok(_) => JSON(json!({ "status": "ok", "id": id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}
