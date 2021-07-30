// src/models.rs

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use std::str::FromStr;

use sqlx::postgres::PgRow;
use warp::reply::Response;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct InsertablePerson {
    pub first_name: String,
    pub last_name: String,
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
    /*
       pub fn add_person(&self, pool: &PgPool) -> Result<Person, sqlx::Error> {
           let mut tx = pool.acquire();
           let rec = sqlx::query("INSERT INTO persons (first_name, last_name)
                   VALUES ( $1, $2 )
                   RETURNING id, first_name, last_name;"
           )
               .bind(&pers.first_name)
               .bind(&pers.last_name)
               .map(|row:PgRow| {
                   Person {
                       id: row.get(0),
                       first_name: row.get(1),
                       last_name: row.get(2)
                   }
               })
               .fetch_one(&mut tx)
               .await?;
           tx.commit().await?;

           log::debug!("person added : {:?}", &rec);
           Ok(rec)
       }

    */
}

impl fmt::Display for InsertablePers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n,{},\n, {}, \n", self.last_name, self.first_name)
    }
}

impl FromStr for InsertablePerson {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InsertablePerson {
            first_name: "".to_string(),
            last_name: "".to_string(),
        })
    }
}

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow, Debug, Eq, PartialEq)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

// si on veut une sortie String et non Json ...
// donc pas très utile.
impl warp::reply::Reply for Person {
    fn into_response(self) -> Response {
        Response::new(
            format!(
                "id: {}\n nom: {}\n prenom: {}",
                self.id, self.first_name, self.last_name
            )
            .into(),
        )
    }
}

impl Default for Person {
    fn default() -> Self {
        Self {
            id: 0,
            last_name: " ".into(),
            first_name: " ".into(),
        }
    }
}

impl warp::reply::Reply for InsertablePerson {
    fn into_response(self) -> Response {
        Response::new(format!("nom: {}\n prenom: {}", self.first_name, self.last_name).into())
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
