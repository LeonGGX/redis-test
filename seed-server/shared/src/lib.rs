// /shared/lib.rs

use serde::{Serialize, Deserialize};

pub type PersonId  = usize;
pub type ListPersons = Vec<Person>;

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

impl Default for Data {
    fn default() -> Self {
        Self {
            list_persons: vec![],
        }
    }
}
