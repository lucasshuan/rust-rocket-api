use crate::{
    database::DB,
    model::{Person, Rocket},
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[get("/rocket/<path>/persons")]
pub fn get_rocket_persons(db: &State<DB>, path: String) -> Result<Json<Vec<Person>>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.person.find_in_rocket(id);
    match result {
        Ok(persons) => Ok(Json(persons)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/rocket/<path>")]
pub fn get_rocket(db: &State<DB>, path: String) -> Result<Json<Rocket>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.rocket.find_by_id(id);
    match result {
        Ok(rocket) => Ok(Json(rocket)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/rocket")]
pub fn get_rockets(db: &State<DB>) -> Result<Json<Vec<Rocket>>, Status> {
    match db.rocket.find_all() {
        Ok(rockets) => Ok(Json(rockets)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/rocket", data = "<new_rocket>")]
pub fn create_rocket(
    db: &State<DB>,
    new_rocket: Json<Rocket>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Rocket {
        id: None,
        name: new_rocket.name.to_owned(),
    };
    let result = db.rocket.create(data);
    match result {
        Ok(rocket) => Ok(Json(rocket)),
        Err(_) => Err(Status::InternalServerError),
    }
}
