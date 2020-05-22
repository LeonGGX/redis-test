// /src/database/db.rs

use std::ops::Deref;

use r2d2::{Pool,
           PooledConnection,
           Error as RDError,
};
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{Connection as RQLConn, Result as SQResult, params, Error as SQError, NO_PARAMS};

use dotenv::dotenv;
use std::env;

use crate::models::person::{Person, InsertablePerson};
use crate::errors::errors::Error::{DBQueryError};
use std::borrow::Borrow;

pub type SQPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub struct Conn{
    pub in_conn: Connection
}


impl Conn {
    pub fn get_list_persons(&self) -> Result<Vec<Person>, DBQueryError> {
        return_all_persons(&self.in_conn)
    }

    pub fn add_person(&self, pers: Person) -> Result<Person, SQError> {
        add_one_person(&self, pers)
    }

    pub fn modify_person(&self, id: i32, pers: Person) -> Result<(), SQError> {
        modify_person(&self.in_conn, id, pers)
    }

    pub fn show_one_person(&self, id: i32) -> Result<Person, SQError> {
        let pers = return_one_person(&self.in_conn, id);
        pers
    }
}

/*
    When Conn is dereferencd, return the sqlite connection.
*/
impl Deref for Conn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.in_conn
    }
}

///
/// Initiate r2d2 pool for connection
///
/// param : db_name : the name of db
///
pub fn init_pool() -> SQPool {

    dotenv().ok(); // This will load our .env file.
    let db_name = env::var("DB_NAME")
    .expect("DB_NAME must be set");

    let manager = SqliteConnectionManager::file(db_name.clone());

    match SQPool::new(manager) {
        Ok(pool) => {
            println!("Pool de Connections ouvert vers: {}", db_name);
            pool
        },
        Err(e)=> panic!("Error: failed to create sqlite pool {}", e),
    }
}

///
/// Opens a rusqlite connection without pool
///
pub fn open_connection() -> Result<RQLConn, SQError> {
    dotenv().ok(); // This will load our .env file.
    let db_name = env::var("DB_NAME")
        .expect("DB_NAME must be set");

    let conn = RQLConn::open(&db_name)?;
    //let conn = Connection::open(&db_name)?;
    println!("Connection ouverte vers: {}", db_name);
    //show_tables(&conn)?;

    Ok(conn)
}

///
/// Opens a pool connection
/// Returns a Conn struct with a Connection type
/// or an error
///
pub fn open_pool_connection() -> Result<Conn, r2d2::Error> {
    let pool = init_pool();
    let db: Result<Connection, RDError> = pool.get();
    Ok(Conn{in_conn:db?})
}

/********************************************/
//
// Returning PERSONS
//
/********************************************/

///
/// Returns the list of persons in a Vec<Person>
///
/// argument:
/// ref to a Connection type
///
//pub fn return_all_persons(conn: &RQLConn) -> Result<Vec<Person>, SQError> {
pub fn return_all_persons(conn: &Connection) -> Result<Vec<Person>, SQError> {

    let mut stmt = conn.prepare("select persons.id,
                                        persons.last_name,
                                        persons.first_name
                                        from persons;")?;
    let persons_iter =
        stmt.query_map(params![],
                        |row| {
                            Ok(Person{
                                id:row.get(0)?,
                                first_name: row.get(1)?,
                                last_name: row.get(2)?,
                            })
                        })?;

    let mut all_persons: Vec<Person> = Vec::new();
    for person in persons_iter {
        all_persons.push(person.unwrap())
    }
    Ok(all_persons)
}

///
/// Returns the list of persons in a Vec<String>
///
/// argument :
/// ref to a Connection type
///
//pub fn all_persons_string(conn: &RQLConn) ->Result<Vec<String>, SQError> {
pub fn all_persons_string(conn: &Connection) ->Result<Vec<String>, SQError> {

    let pers = return_all_persons(&conn)?;
    let mut pers_str: Vec<String> = Vec::new();

    for pe in pers {
        //let mut str = pe.all_data_to_string();
        let str = pe.to_string();
        pers_str.push(str);
    }
    Ok(pers_str)
}

///
/// Returns one Person from the DB
///
/// used to show the person
/// not to modify
///
/// arguments:
/// ref to a Connection type
/// id of the Person to show
///
pub fn return_one_person(conn: &Connection, id: i32) -> Result<Person, SQError> {

    let mut stmt = conn.prepare("select persons.id,
                                                      persons.last_name,
                                                      persons.first_name
                                               from persons
                                               where persons.id = :id;")?;
    let pers = stmt.query_row_named(&[(":id", &id)],
                                             |row|
                                                 Ok(Person{
                                                    id:row.get(0)?,
                                                    first_name: row.get(1)?,
                                                    last_name: row.get(2)?,
                                                 }))?;

    println!("{:?}", &pers);

    Ok(pers)
}

/***********
*  CRUD
*  Add
*  Modify
*  Delete
* ***********/
///
/// Add a person to the DB
///
/// uses NewPerson instead of Person not to force the id
///
/// arguments :
/// ref to a Connection type,
/// Person to add,
///
//pub fn add_one_person(conn: &RQLConn, person: InsertablePerson) -> Result<(), SQError> {
pub fn add_one_person(conn: &Conn, in_person: Person) -> Result<Person, SQError> {

    let insertable_person = InsertablePerson::from_person(in_person.clone());
    conn.execute("insert into persons(last_name, first_name) values (?1, ?2);",
                 params![insertable_person.last_name, insertable_person.first_name],)?;
    println!("Nouvelle personne créée");
    Ok(in_person)
}


///
/// Modify Person
///
/// Modify one person in the DB Person
/// uses the id of the Person
/// 
/// arguments :
/// rusqlite connection
/// id of the Person to find
/// ref to a new Person with the modified data
///
pub fn modify_person(conn: &Connection, pers_id: i32, new_pers: Person) -> Result<(), SQError> {
    conn.execute("UPDATE persons SET last_name = ?1, first_name = ?2 WHERE id = ?3;",
                 params![new_pers.last_name, new_pers.first_name, pers_id])?;
    println!("Personne modifiée !");
    Ok(())
}

///
/// Delete one Person from DB
///
/// arguments :
/// rusqlite connection
/// id of the Person to find
///
pub fn delete_person(conn: &Connection, pers_id: i32) -> Result<(), SQError> {
   unimplemented!()
}
