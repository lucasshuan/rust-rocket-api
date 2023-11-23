use crate::{database::DB, model::Person};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[get("/person/<path>")]
pub fn get_person(db: &State<DB>, path: String) -> Result<Json<Person>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.person.find_by_id(&id);
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

#[put("/person/<path>", data = "<new_person>")]
pub fn update_person(
    db: &State<DB>,
    path: String,
    new_person: Json<Person>,
) -> Result<Json<Person>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Person {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_person.name.to_owned(),
        age: new_person.age.to_owned(),
        job: new_person.job.to_owned(),
        rocket_id: new_person.rocket_id.to_owned(),
    };
    let update_result = db.person.update(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_person_info = db.person.find_by_id(&id);
                return match updated_person_info {
                    Ok(person) => Ok(Json(person)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/person/<path>")]
pub fn delete_person(db: &State<DB>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.person.delete(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Person successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
