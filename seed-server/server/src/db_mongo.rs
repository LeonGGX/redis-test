// shared/src/db/db_mongo.rs

use bson::oid::ObjectId;
use bson::{doc, from_bson, to_bson, Bson, Document};

use crate::errors::MyError;
use crate::person::{InsertablePers, Person};

use mongodb::{Client, Collection};
use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

pub(crate) type Pool = r2d2::Pool<MongodbConnectionManager>;
pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

/*
    create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
*/
pub fn init_pool() -> Pool {
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
            .with_db("local")
            .build(),
    );
    match Pool::builder().max_size(8).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create mongodb pool {}", e),
    }
}

pub fn open_pool_connection() -> Result<Conn, r2d2::Error> {
    let pool = init_pool();
    let db = pool.get();
    Ok(Conn(db?))
}

pub fn get_collection() -> Result<Collection, MyError> {
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let db = client.database("local");
    let collection = db.collection("Persons");
    Ok(collection)
}

pub fn add_person(pers: Person) -> Result<Person, MyError> {
    let coll = get_collection()?;
    let insertable = InsertablePers::from_person(pers);
    let ret_val = insertable.clone();
    let value = doc! {"nom" : insertable.nom, "prenom" : insertable.prenom};
    let result = coll.insert_one(value, None)?;

    let res = bson::from_bson(result.inserted_id);

    match res {
        Ok(res) => {
            let added_person = Person {
                id: res,
                nom: ret_val.nom,
                prenom: ret_val.prenom,
            };
            Ok(added_person)
        }
        Err(err) => Err(err.into()),
    }
}

pub fn get_list_persons() -> Result<Vec<Person>, MyError> {
    let cursor = get_collection()?.find(None, None)?;
    let res: Result<Vec<_>, _> = cursor
        .map(|row| row.and_then(|item| Ok(from_bson::<Person>(bson::Bson::Document(item))?)))
        .collect();
    Ok(res?)
}

pub fn get_person_by_id(pers_id: &str) -> Result<Option<Person>, MyError> {
    let coll = get_collection()?;
    let cursor: Option<Document> =
        coll.find_one(Some(doc! { "_id": ObjectId::with_string(pers_id)?}), None)?;
    cursor
        .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
        .map_or(Ok(None), |v| v.map(Some))
}

pub fn modify_person_by_id(
    pers_id: &str,
    modifyed_person: Person,
) -> Result<Option<Person>, MyError> {
    let coll = get_collection()?;
    let cursor: Option<Document> = coll.find_one_and_replace(
        doc! {"_id": ObjectId::with_string(pers_id)?},
        doc! {"_id": ObjectId::with_string(pers_id)?,
        "nom" : modifyed_person.nom,
        "prenom" : modifyed_person.prenom },
        Some(Default::default()),
    )?;
    cursor
        .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
        .map_or(Ok(None), |v| v.map(Some))
}

pub fn delete_person(pers: Person) -> Result<Option<Person>, MyError> {
    let coll = get_collection()?;
    if let Bson::Document(doc) = to_bson(&pers)? {
        let cursor: Option<Document> = coll.find_one_and_delete(doc, Some(Default::default()))?;
        cursor
            .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
            .map_or(Ok(None), |v| v.map(Some))
    } else {
        Ok(None)
    }
}
