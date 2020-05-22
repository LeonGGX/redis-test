// src/models/person.rs

use crate::db::local_store;
use serde::{Serialize, Deserialize};
//use seed::prelude::IndexMap;

//use uuid::Uuid;

//type PersonId  = Uuid;
pub(crate) type PersonId  = usize;

//pub(crate) type ListPersons = IndexMap<PersonId, Person>;
pub(crate) type ListPersons = Vec<Person>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Person {
    pub fn new(id: i32, first_name: String, last_name: String) -> Person {
        Person { id, first_name, last_name }
    }
}

impl Default for Person {
    fn default()-> Self {
        Person {
            id: 0,
            first_name: " ".into(),
            last_name: " ".into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Data {
    pub list_persons : ListPersons,
}

impl Data{
    pub fn store(&self) {
        local_store::store_data(self);
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            list_persons: vec![],
        }
    }
}


