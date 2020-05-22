// /src/database/db_pg

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;
use std::env;

use crate::models::person::{Person, InsertablePerson};

use crate::database::schema::persons::dsl::*;

type DRError = diesel::result::Error;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pg_pool() -> Result<DbPool, diesel::ConnectionError>  {

    dotenv().ok(); // This will load our .env file.
    let db_name = env::var("DATABASE_PG")
        .expect("DATABASE_PG must be set");

    let manager =
        ConnectionManager::<PgConnection>::new(db_name);

    // set up database connection pool
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    Ok(pool)
}
