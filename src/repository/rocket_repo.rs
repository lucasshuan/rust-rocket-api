use crate::model::Rocket;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    sync::Collection,
};

#[derive(Clone)]
pub struct RocketRepo {
    col: Collection<Rocket>,
}

impl RocketRepo {
    pub fn new(col: Collection<Rocket>) -> Self {
        RocketRepo { col }
    }

    pub fn find_by_id(&self, id: String) -> Result<Rocket, Error> {
        let rocket = self
            .col
            .find_one(Some(doc! {"id": id}), None)
            .ok()
            .expect("Error finding rocket");
        Ok(rocket.unwrap())
    }

    pub fn find_all(&self) -> Result<Vec<Rocket>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error finding rockets");
        let persons = cursors.map(|doc| doc.unwrap()).collect();
        Ok(persons)
    }

    pub fn create(&self, new_rocket: Rocket) -> Result<InsertOneResult, Error> {
        let new_doc = Rocket {
            id: None,
            name: new_rocket.name,
        };
        let person = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating rocket");
        Ok(person)
    }
}
