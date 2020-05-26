// src/handlers/person_handlers.rs
use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};

use crate::db_mongo;
use crate::AppState;
use shared::person::{ListPersons, Person};

pub async fn simple_index(data: web::Data<Mutex<AppState>>) -> String {
    let app_name = &data.lock().unwrap().app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

pub async fn list_persons_str(_state: web::Data<Mutex<AppState>>) -> impl Responder {
    //let conn = &state.lock().unwrap().conn;
    //let vec_pers = conn.get_list_persons().unwrap();

    let vec_pers = db_mongo::get_list_persons().unwrap();

    let str_pers: ListPersons = ListPersons::new(vec_pers);
    let str = str_pers.vec_to_string();

    HttpResponse::Ok().body(str)
}

pub async fn list_persons_json(_state: web::Data<Mutex<AppState>>) -> impl Responder {
    let res = db_mongo::get_list_persons().unwrap();
    HttpResponse::Ok().json(res)
}

pub async fn list_persons_json_from_list(_state: web::Data<Mutex<AppState>>) -> impl Responder {
    /*
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
     */
    let res = db_mongo::get_list_persons().unwrap();
    let list = ListPersons::new(res);

    HttpResponse::Ok().json(list)
}

pub async fn add_person_hdl(
    _state: web::Data<Mutex<AppState>>,
    pers: web::Json<Person>,
) -> impl Responder {
    let my_person = pers.into_inner();
    let new_person = db_mongo::add_person(my_person);
    HttpResponse::Ok().json(new_person.unwrap())
}

pub async fn show_one_person_id(id: web::Path<String>) -> impl Responder {
    let in_id = id.into_inner();
    let found_person = Some(db_mongo::get_person_by_id(&in_id).unwrap());
    HttpResponse::Ok().json(found_person)
}

pub async fn modify_person_hdl(
    id: web::Path<String>,
    modifyed_person: web::Json<Person>,
) -> impl Responder {
    let in_id = id.into_inner();
    let mod_pers = modifyed_person.into_inner();

    let succes = Some(db_mongo::modify_person_by_id(&in_id, mod_pers).unwrap());
    HttpResponse::Ok().json(succes)
}

pub async fn delete_person_hdl(delete_pers: web::Json<Person>) -> impl Responder {
    let del_pers = delete_pers.into_inner();
    let succes = Some(db_mongo::delete_person(del_pers).unwrap());
    HttpResponse::Ok().json(succes.unwrap())
}
