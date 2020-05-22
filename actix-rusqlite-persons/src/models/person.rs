// src/models/person.rs

use serde::{Serialize, Deserialize};

pub(crate) type PersonId  = usize;
pub(crate) type ListPersons = Vec<Person>;

// if use sqlx
use sqlx::{/*PgPool,*/ FromRow, /*Row*/};


use crate::database::schema::persons;

#[derive(Debug, Clone, Serialize, Queryable, Deserialize, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Person {
    pub fn new(id: i32, first_name: String, last_name: String) -> Person {
        Person { id, first_name, last_name }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.id.to_string());
        str.push_str(&self.last_name);
        str.push_str(" ");
        str.push_str(&self.first_name);
        str
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

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "persons"]
pub struct InsertablePerson {
    pub first_name : String,
    pub last_name : String,
}

impl InsertablePerson {
    pub fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.last_name);
        str.push_str(" ");
        str.push_str(&self.first_name);
        str
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Data {
    pub list_persons : ListPersons,
}

impl Data{
    //pub fn store(&self) {
        //local_store::store_data(self);
    //}

    pub fn new (persons: Vec<Person>) -> Self {
        Self{list_persons: persons}
    }

    pub fn to_vec_string(&self) -> Vec<String> {
        let list = self.clone();
        let mut vec_str: Vec<String> = Vec::new();
        for pers in list.list_persons {
            vec_str.push(InsertablePerson::from_person(pers).to_string());
        }
        vec_str
    }

    pub fn vec_to_string(&self) -> String {
        let mut str = String::new();
        let str_vec = self.to_vec_string();
        for pers in str_vec {
            str.push_str(pers.as_ref());
            str.push("\n".parse().unwrap());
        }
        str
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            list_persons: vec![],
        }
    }
}


