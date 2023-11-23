mod api;
mod database;
mod model;
mod repository;

#[macro_use]
extern crate rocket;

use api::{
    person_api::{create_person, get_person, get_persons},
    rocket_api::{create_rocket, get_rocket, get_rocket_persons, get_rockets},
};
use database::DB;

#[launch]
fn rocket() -> _ {
    let db = DB::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_person])
        .mount("/", routes![get_persons])
        .mount("/", routes![get_person])
        .mount("/", routes![create_rocket])
        .mount("/", routes![get_rockets])
        .mount("/", routes![get_rocket])
        .mount("/", routes![get_rocket_persons])
}
