// mongo_connection.rs

//use dotenv::dotenv;

use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager,};
//use std::env;
use std::ops::Deref;
use mongodb::{
    error::Error as MongoError,
    Client,
    Collection,
};
use actix_web::{FromRequest, Error};
use r2d2_mongodb::r2d2::State;

use actix::prelude::*;

use crate::person::*;


type Pool = r2d2::Pool<MongodbConnectionManager>;
pub type DbConnection = r2d2::PooledConnection<MongodbConnectionManager>);

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

pub fn connection() -> Result<DbConnection, r2d2::Error> {
    let pool = init_pool();
    let db = pool.get();
    Ok(DbConnection(db?))
}

pub fn get_collection() -> Result<Collection, MongoError> {
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let db = client.database("local");
    let collection = db.collection("Persons");
    Ok(collection)
}

/*
    When Conn is dereferencd, return the mongo connection.
*/
impl Deref for DbConnection {
    type Target = PooledConnection<MongodbConnectionManager>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DbConnection {
    pub fn get_collection(&self) -> Result<Collection, MongoError> {
        //let client = Client::with_uri_str("mongodb://localhost:27017/").unwrap();
        //let db = client.database("local");

        let collection = self.get_collection();
        collection
    }

    pub fn get_list_persons(&self) -> Result<Vec<Person>, MongoError> {
        let cursor = self.get_collection()?.find(None, None)?;
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
}