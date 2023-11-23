use crate::model::Person;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::Collection,
};

#[derive(Clone)]
pub struct PersonRepo {
    col: Collection<Person>,
}

impl PersonRepo {
    pub fn new(col: Collection<Person>) -> Self {
        PersonRepo { col }
    }

    pub fn find_by_id(&self, id: String) -> Result<Person, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let person = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error finding person");
        Ok(person.unwrap())
    }

    pub fn find_in_rocket(&self, rocket_id: String) -> Result<Vec<Person>, Error> {
        let obj_id = ObjectId::parse_str(rocket_id).unwrap();
        let filter = doc! {"rocket_id": obj_id};
        let cursors = self
            .col
            .find(filter, None)
            .ok()
            .expect("Error getting list of persons");
        let persons = cursors.map(|doc| doc.unwrap()).collect();
        Ok(persons)
    }

    pub fn find_all(&self) -> Result<Vec<Person>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of persons");
        let persons = cursors.map(|doc| doc.unwrap()).collect();
        Ok(persons)
    }

    pub fn create(&self, new_person: Person) -> Result<InsertOneResult, Error> {
        let new_doc = Person {
            id: None,
            age: new_person.age,
            name: new_person.name,
            job: new_person.job,
            rocket_id: new_person.rocket_id,
        };
        let person = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating person");
        Ok(person)
    }
}
