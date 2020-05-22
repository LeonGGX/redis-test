// src/models/person-old-client

use serde_derive::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub nom: String,
    pub prenom: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertablePerson {
    pub nom : String,
    pub prenom : String,
}

impl InsertablePerson {
    pub fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            nom: person.nom,
            prenom: person.prenom,
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.nom);
        str.push_str(" ");
        str.push_str(&self.prenom);
        str
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPersons {
    pub list_pers: Vec<Person>,
}

impl ListPersons {
    pub fn new(vec_pers: Vec<Person>) -> Self {
        ListPersons{list_pers: vec_pers}
    }

    pub fn to_vec_string(&self) -> Vec<String> {
        let list = self.clone();
        let mut vec_str: Vec<String> = Vec::new();
        for pers in list.list_pers {
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Data {
    pub list_persons : Vec<Person>,
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

