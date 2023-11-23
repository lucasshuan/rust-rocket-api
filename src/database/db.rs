use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::model::Person;
use crate::model::Rocket;
use crate::repository::PersonRepo;
use crate::repository::RocketRepo;
use mongodb::sync::{Client, Collection};

#[derive(Clone)]
pub struct DB {
    pub person: PersonRepo,
    pub rocket: RocketRepo,
}

impl DB {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");

        let persons_col: Collection<Person> = db.collection("Person");
        let rockets_col: Collection<Rocket> = db.collection("Dog");

        let person = PersonRepo::new(persons_col);
        let rocket = RocketRepo::new(rockets_col);

        DB { person, rocket }
    }
}
