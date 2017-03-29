use rocket_contrib::{JSON, Value};
use models::{Visitor, NewVisitor};
use db::{DB, visitor, Error};


#[get("/visitor")]
pub fn get_all(db: DB) -> JSON<Vec<Visitor>> {
    JSON(visitor::get(db.conn(), None))
}


#[get("/visitor/<id>")]
pub fn get(db: DB, id: i32) -> Option<JSON<Visitor>> {
    let mut visitors = visitor::get(db.conn(), Some(id));
    visitors.pop().map(|v| JSON(v))
}


#[post("/visitor/create", format="application/json", data="<visitor>")]
pub fn create(db: DB, visitor: JSON<NewVisitor>) -> JSON<Value> { // returns id
    let visitor = visitor::create(db.conn(), &visitor.name, &visitor.mail, visitor.site.clone());
    match visitor {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        _ => JSON(json!({ "status": "database error" }))
    }
}


#[post("/visitor/<id>", format="application/json", data="<visitor>")]
pub fn update(db: DB, id: i32, visitor: JSON<NewVisitor>) -> JSON<Value> { // returns id
    let visitor = visitor::update(db.conn(), id, &visitor.name, &visitor.mail, visitor.site.clone());
    match visitor {
        Ok(p) => JSON(json!({ "status": "ok", "id": p.id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}


#[delete("/visitor/<id>")]
pub fn delete(db: DB, id: i32) -> JSON<Value> {
    let num = visitor::delete(db.conn(), id);
    match num {
        Ok(_) => JSON(json!({ "status": "ok", "id": id })),
        Err(Error::RecordNotFound) => JSON(json!({ "status": "error", "description": "not found" })),
        _ => JSON(json!({ "status": "error", "description": "database error" }))
    }
}
