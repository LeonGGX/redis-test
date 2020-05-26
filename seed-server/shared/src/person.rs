// shared/src/models/person.rs

use core::fmt;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Person {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub nom: String,
    pub prenom: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            id: None,
            nom: " ".into(),
            prenom: " ".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertablePers {
    pub nom: String,
    pub prenom: String,
}

impl InsertablePers {
    pub fn from_person(person: Person) -> Self {
        Self {
            nom: person.nom,
            prenom: person.prenom,
        }
    }
}

impl fmt::Display for InsertablePers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n,{},\n, {}, \n", self.nom, self.prenom)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ListPersons {
    pub list_persons: Vec<Person>,
}

impl ListPersons {
    pub fn new(vec_pers: Vec<Person>) -> Self {
        Self {
            list_persons: vec_pers,
        }
    }

    pub fn to_vec_string(&self) -> Vec<String> {
        let list = self.clone();
        let mut vec_str: Vec<String> = Vec::new();
        for pers in list.list_persons {
            vec_str.push(InsertablePers::from_person(pers).to_string());
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

impl Default for ListPersons {
    fn default() -> Self {
        Self {
            list_persons: vec![],
        }
    }
}
