use crate::model::Rocket;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
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

    pub fn find_by_id(&self, id: &String) -> Result<Rocket, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let rocket = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error finding rocket");
        Ok(rocket.unwrap())
    }

    pub fn find_all(&self) -> Result<Vec<Rocket>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of persons");
        let rockets = cursors.map(|doc| doc.unwrap()).collect();
        Ok(rockets)
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

    pub fn update(&self, id: &String, new_rocket: Rocket) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":	{
                "id": new_rocket.id,
                "name": new_rocket.name,
            },
        };
        let rocket = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating rocket");
        Ok(rocket)
    }

    pub fn delete(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let result = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting rocket");
        Ok(result)
    }
}
