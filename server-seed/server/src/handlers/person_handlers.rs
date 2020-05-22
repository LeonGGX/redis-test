// person_handleers.rs

use actix_web::{
    web,
    HttpResponse,
    Responder,
};
use mongodb::error::Error as MongoError;

use std::sync::Mutex;

use crate::AppState;
//use crate::db::mongo_connection::Conn;
use crate::db::db_mongo;
use crate::models::person::{Person, ListPersons};



pub async fn simple_index(data: web::Data<Mutex<AppState>>) -> String {
    let app_name = &data.lock().unwrap().app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}


pub async fn list_persons_str(state: web::Data<Mutex<AppState>>) -> impl Responder {

    let conn = &state.lock().unwrap().conn;
    let vec_pers = conn.get_list_persons().unwrap();
    let str_pers: ListPersons = ListPersons::new(vec_pers);
    let str = str_pers.vec_to_string();

    HttpResponse::Ok().body(str)
}

pub async fn list_persons_json(state: web::Data<Mutex<AppState>>) -> impl Responder {

    let conn = &state.lock().unwrap().conn;
    let coll = conn.get_collection().unwrap().find(None, None).unwrap();
    let res = coll
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(e) => Err(e.into()),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Person>, MongoError>>();

    HttpResponse::Ok().json(res.unwrap())
}


pub async fn list_persons_json_from_list(state: web::Data<Mutex<AppState>>) -> impl Responder {

    let conn = &state.lock().unwrap().conn;
    let coll = conn.get_collection().unwrap().find(None, None).unwrap();

    let res = coll
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(e) => Err(e.into()),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Person>, MongoError>>();
    let list = ListPersons::new(res.unwrap());

    HttpResponse::Ok().json(list)
}


pub async fn add_person(pers: web::Json<Person>) -> impl Responder {
    let my_person = pers.into_inner();
    if let new_person = db_mongo::add_person(my_person){
        HttpResponse::Ok().json(new_person.unwrap())
    } else {
        HttpResponse::InternalServerError().body("nouvelle personne pas ajoutée")
    }
}

pub async fn show_one_person(pers: web::Json<Person>) -> impl Responder {
    let my_person = pers.into_inner();
    if let found_person = Some(db_mongo::find_one(&my_person)) {
        HttpResponse::Ok().json(found_person)
    } else {
        HttpResponse::BadRequest().body("Pas trouvé")
    }
}