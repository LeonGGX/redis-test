// main.rs

// import standart
use std::sync::{Mutex};

// import actix_web
use actix_web::{
    web,
    App,
    HttpServer,
    middleware::{Logger},
};

use env_logger;

// import driver mongodb
use mongodb::error::Error as MongoError;


// les différents modules qui correspondent aux sous-dossiers


mod handlers;
mod errors;

// import des fichiers internes
use crate::db::mongo_connection::Conn;
use crate::handlers::person_handlers::*;
use shared;
use server_seed::shared::models::person;


///
/// la structure AppState permet de mettre des données
/// accessibles partout
///
pub struct AppState {
    app_name: String,
    conn: Conn,
}

///
/// la fonction main
/// avec la macro actix_rt::main
/// qui est le runtime actix
///
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    type Error = MongoError;

    // une fonction set_var qui permet de définir ce qui apparaît dans la console
    // ici le journal RUST LOG
    // les infos provenant du serveur, de actix_web et actix_http
    // puis on lance avec env_logger::init()
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,actix_http=trace");
    env_logger::init();

    // initialisation de la connection avec la base de données mongodb
    let new_conn = db::mongo_connection::open_pool_connection().unwrap();

    // initialisation des web::Data
    // en fait on initialise la struct AppState, sous forme de Mutex
    // pourra être utilisée partout dans l'application
    // c'est par l'AppState qu'on passe la connection à la DB
    let new_data =
        web::Data::new(
        Mutex::new(
            AppState{
                    app_name: String::from("Application de Léon en Actix"),
                    conn: new_conn,
            })
        );

    HttpServer::new(  move || {
        App::new()
            .wrap(Logger::default())
            .app_data(new_data.clone())
            .route("/", web::get().to(simple_index))
            .route("/string",web::get().to(list_persons_str))
            .route("/json", web::get().to(list_persons_json))
            .route("/json", web::post().to(add_person))
            .route("/json_list", web::get().to(list_persons_json_from_list))
            .route("/json/{_id}", web::get().to(show_one_person))
    })
        .workers(2)
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

///
/// les tests
/// ici test sur la création d'une nouvelle personne dans la DB
/// 
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    use crate::models::person::Person;

    #[actix_rt::test]
    async fn test_add_person() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new().service(web::resource("/json").route(web::post().to(add_person))),
        ).await;

        let req = test::TestRequest::post()
            .uri("/json")
            .set_json(&Person {
                id: None,
                nom: "DOE".to_owned(),
                prenom: "John".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        println!("reponse : {:?}", response_body);

        //assert_eq!(response_body, r##"{"nom":"my-name","prenom":"my-prenom"}"##);

        Ok(())
    }
}
