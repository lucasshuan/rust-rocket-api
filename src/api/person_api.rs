use crate::{database::DB, model::Person};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[get("/person/<path>")]
pub fn get_person(db: &State<DB>, path: String) -> Result<Json<Person>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.person.find_by_id(id);
    match result {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/person")]
pub fn get_persons(db: &State<DB>) -> Result<Json<Vec<Person>>, Status> {
    match db.person.find_all() {
        Ok(persons) => Ok(Json(persons)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/person", data = "<new_person>")]
pub fn create_person(
    db: &State<DB>,
    new_person: Json<Person>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Person {
        id: None,
        name: new_person.name.to_owned(),
        age: new_person.age.to_owned(),
        job: new_person.job.to_owned(),
        rocket_id: new_person.rocket_id.to_owned(),
    };
    let result = db.person.create(data);
    match result {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(Status::InternalServerError),
    }
}
