// src/db/db_mongo

use bson::{doc, Document, from_bson, Bson, to_bson};
use bson::oid::ObjectId;

use crate::models::person::{Person, InsertablePerson,};
use crate::db::mongo_connection;
use crate::errors::errors::MyError;


pub fn add_person(pers: Person) -> Result<Person, MyError> {

    let coll = mongo_connection::get_collection()?;
    let insertable = InsertablePerson::from_person(pers.clone());
    let ret_val = insertable.clone();
    let value = doc! {"nom" : insertable.nom, "prenom" : insertable.prenom};
    let result = coll.insert_one(value, None)?;

    let res = bson::from_bson(result.inserted_id);

    match res {
        Ok(res) => {
            let added_person = Person{
            id: res,
            nom: ret_val.nom,
            prenom: ret_val.prenom};
        Ok(added_person)},
        Err(err) => Err(err.into())
    }
}


pub fn get_list_persons() -> Result<Vec<Person>, MyError> {

    let cursor = mongo_connection::get_collection()?.find(None, None)?;
    let res: Result<Vec<_>,_> = cursor
        .map(|row|row.and_then(|item|Ok(from_bson::<Person>(bson::Bson::Document(item))?)))
        .collect();
    Ok(res?)
}


pub fn get_person_by_id(pers_id: &str) -> Result<Option<Person>, MyError> {

    let coll = mongo_connection::get_collection()?;
    let cursor: Option<Document> =
        coll.find_one(
            Some(doc! { "_id": ObjectId::with_string(pers_id)?}),
            None
        )?;
    cursor
        .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
        .map_or(Ok(None),|v| v.map(Some))
}

pub fn modify_person_by_id(pers_id: &str, modifyed_person: Person) -> Result<Option<Person>, MyError> {

    let coll = mongo_connection::get_collection()?;
    let cursor : Option<Document> =
        coll.find_one_and_replace(
            doc! {"_id": ObjectId::with_string(pers_id)?},
            doc! {"_id": ObjectId::with_string(pers_id)?,
                            "nom" : modifyed_person.nom,
                            "prenom" : modifyed_person.prenom },
            Some(Default::default())
        )?;
    cursor
        .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
        .map_or(Ok(None), |v|v.map(Some))
}

pub fn delete_person(pers: Person) -> Result<Option<Person>, MyError> {

    let coll = mongo_connection::get_collection()?;
    if let Bson::Document(mut doc) = to_bson(&pers)? {
        let cursor: Option<Document> =
            coll.find_one_and_delete(doc, Some(Default::default()))?;
        cursor
            .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
            .map_or(Ok(None), |v|v.map(Some))
    } else {
        Ok(None)
    }
}


