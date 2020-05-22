// src/db/db_mongo

use mongodb::{
    error::Error as MongoError,
};

use bson::doc;

use crate::models::person::{Person, InsertablePerson,};
use crate::db::mongo_connection;
/*
pub fn build_list_persons() -> Vec<bson::ordered::OrderedDocument> {

    let persons = vec![
        doc!{"nom" : "GENGOUX", "prenom": "Léon"},
        doc!{"nom" : "FRANQUET", "prenom": "Véronique"},
        doc!{"nom" : "EINSTEIN", "prenom": "Albert"},
        doc!{"nom": "MOZART", "prenom" : "Wolfgang-Amadeus"},
    ];
    persons
}
*/
pub fn add_person(pers: Person) -> Result<Person, MongoError> {

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

/*
pub fn get_list_persons() -> Result<Vec<Person>, MongoError> {
    let cursor = mongo_connection::get_collection()?.find(None, None)?;
    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(e) => Err(e.into()),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Person>, MongoError>>()
}
*/
/*
pub fn get_person_by_id(id: &str) -> Result<Option<Person>, MongoError> {
    let coll = mongo_connection::get_collection()?;
    //let cursor =
        coll.find_one(
            Some(doc! { "_id": ObjectId::with_string(id).unwrap() }),
            cursor
                .map(|doc| Ok(bson::from_bson::<Person>(bson::Bson::Document(doc))?))
                .map_or(Ok(None),|v| v.map(Some))
        )
}
*/
// takes a borrowed partial model and returns the model if it matches or None if it doesn't
// returns a bool instead of error types
pub fn find_one(model: &Person) -> Result<Option<Person>, bool> {
    let coll = mongo_connection::get_collection().unwrap();
    match bson::to_bson(model) {
        Ok(model_bson) => {
            match model_bson{
                bson::Bson::Document(model_doc) => {
                    match coll.find_one( Some(model_doc), None) {
                        Ok(db_result) => {
                            match db_result {
                                Some(result_doc) => {
                                    match bson::from_bson(bson::Bson::Document(result_doc)) {
                                        Ok(result_model) => {
                                            return Ok(Some(result_model))
                                        },
                                        Err(_err) => {
                                            println!("failed to get model from bson");
                                            return Err(false)
                                        }
                                    }
                                },
                                None => {
                                    println!("No model found");
                                    return Ok(None)
                                }
                            }
                        },
                        Err(err) => {
                            println!("Failed to delete doc from database:\n{}",err);
                            return Err(false)
                        }
                    }
                },
                _ => {
                    println!("Failed to create document from new model bson");
                    return Err(false)
                }
            }
        },
        Err(err) => {
            println!("Failed to create bson from model:\n{}",err);
            return Err(false)
        }
    }
}

