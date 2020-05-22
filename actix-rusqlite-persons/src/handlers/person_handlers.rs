// person_handleers.rs

use actix_web::{web, HttpResponse, Responder, Error};

use std::sync::Mutex;

use crate::AppState;
use crate::models::person::{Person, Data as ModelData};

//use crate::database::db_sqlite::*;
//use crate::database::db_diesel_pg::*;
use crate::database::db_sqlx_pg::*;

use futures::FutureExt;


pub async fn simple_index(data: web::Data<Mutex<AppState>>) -> String {
    let app_name = &data.lock().unwrap().app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

///
/// Retourne la liste des Personnes de la DB
/// sous forme de string
///
pub async fn list_persons_str(state: web::Data<Mutex<AppState>>) -> impl Responder {

    // on prend une connection vers la DB
    // dans le pool qui vient par l'AppState
    let pool = &state.lock().unwrap().pool;

    /*
    let conn = pool.get()
        .expect("couldn't get db connection from pool");
    */
    //let conn = &state.lock().unwrap().conn;

    // utilisation de sqlx::Pool<sqlx::PgConnection>
    let conn = pool.conn().await;

    //let vec_pers = get_all_persons(state.clone()).unwrap();
    let vec_pers = Person::find_all(pool).await;
    let str_pers = ModelData::new(vec_pers.unwrap());
    let str = str_pers.vec_to_string();

    HttpResponse::Ok().body(str)
}

///
/// Retourne la liste des Personnes de la DB
/// sous forme de liste d'enregistrements JSON
///
pub async fn list_persons_json(state: web::Data<Mutex<AppState>>) -> impl Responder {

    // on prend une connection vers la DB
    // dans le pool qui vient par l'AppState
    let pool = &state.lock().unwrap().pool;

    let persons = Person::find_all(pool).await;
    HttpResponse::Ok().json(&persons)
}
/*
async fn get_all_persons(state: web::Data<Mutex<AppState>>) -> Result<Vec<Person>, diesel::result::Error>{

    use crate::database::schema::persons::dsl::*;

    let pool = &state.lock().unwrap().pool;

    /*let conn = pool.get()
        .expect("couldn't get db connection from pool");*/



    // utilisation de sqlx::Pool<sqlx::PgConnection>
    let conn = pool.conn().await?;

    let vec_pers = persons.load::<Person>(&conn)?;
    Ok(vec_pers)
}
*/
///
/// Retourne la liste des Personnes de la DB
/// sous forme d'une liste JSON
///
pub async fn list_persons_json_from_list(state: web::Data<Mutex<AppState>>) -> impl Responder {

    // on prend une connection vers la DB
    // dans le pool qui vient par l'AppState
    let pool = &state.lock().unwrap().pool;

    let vec_pers = Person::find_all(pool).await;
    let list = ModelData::new(vec_pers.unwrap());

    HttpResponse::Ok().json(list)
}


pub async fn add_person(state: web::Data<Mutex<AppState>>,
                        pers: web::Json<Person>) -> impl Responder {
    let pool = &state.lock().unwrap().pool;
    let person = Person::create(pers.to_owned(), pool).await;
    HttpResponse::Ok().json(&person)

}




/*
// diesel
fn add_single_person(state: web::Data<Mutex<AppState>>,
                     pers: web::Json<Person>) -> Result<usize, diesel::result::Error> {
    use crate::database::schema::persons::dsl::*;

    let pool = &state.lock().unwrap().pool;
    let conn = pool.get()
        .expect("couldn't get db connection from pool");

    //let conn = &state.lock().unwrap().conn;

    let new_pers = InsertablePerson::from_person(pers.into_inner());
    let res = insert_into(persons)
        .values(&new_pers)
        .execute(&conn)?;
    Ok(res)
}
*/
/*
pub async fn get_person(state: web::Data<Mutex<AppState>>,
                             pers: web::Json<Person>) -> Result<HttpResponse, Error> /*impl Responder*/ {
/*
    // on prend une connection vers la DB
    // dans le pool qui vient par l'AppState
    let pool = &state.lock().unwrap().pool;
    let conn = pool.get()
        .expect("couldn't get db connection from pool");
*/
    let conn = &state.lock().unwrap().conn;

    let my_person = pers.into_inner();
    let id = my_person.id.clone();

     // use web::block to offload blocking Diesel code without blocking server thread
    let person: Option<Person> =
        web::block(move || find_person_by_id(&conn, my_person.id.clone()))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    if let Some(person) = person {
        Ok(HttpResponse::Ok().json(person))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No person found with id: {}", id));
        Ok(res)
    }
}
*/