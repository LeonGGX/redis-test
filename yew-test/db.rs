#![allow(unused_imports)]

use r2d2::{Pool,
           Error as RDError,
};
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{Connection as RQLConn, Result as SQResult, params, Error as SQError, NO_PARAMS};

use dotenv::dotenv;
use std::env;

use crate::Person;
use std::borrow::Borrow;

pub type SQPool = Pool<SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<SqliteConnectionManager>;

///
/// Initiate r2d2 pool for connection
///
/// param : db_name : the name of db
///
pub fn init_pool(db_name: String) -> Result<SQPool, RDError> {
    let manager = SqliteConnectionManager::file(db_name.clone());
    println!("Pool de Connections ouvert vers: {}", db_name);
    SQPool::new(manager)
}

///
/// Opens a rusqlite connection without pool
///
pub fn open_connection() -> Result<RQLConn, SQError> {

    dotenv().ok(); // This will load our .env file.
    let db_name = env::var("DB_NAME")
        .expect("DB_NAME must be set");

    let conn = RQLConn::open(&db_name)?;
    println!("Connection ouverte vers: {}", db_name);
    //show_tables(&conn)?;

    Ok(conn)
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
/// rusqlite connection
///
pub fn return_all_persons(conn: &RQLConn) -> Result<Vec<Person>, SQError> {

    let mut stmt = conn.prepare("select persons.id,
                                        persons.nom,
                                        persons.prenom
                                        from persons;")?;
    let persons_iter =
        stmt.query_map(params![],
                        |row| {
                            Ok(Person{
                                id:row.get(0)?,
                                nom: row.get(1)?,
                                prenom: row.get(2)?,
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
/// rusqlite connection
///
pub fn all_persons_string(conn: &RQLConn) ->Result<Vec<String>, SQError> {

    let pers = return_all_persons(&conn)?;
    let mut pers_str: Vec<String> = Vec::new();

    for pe in pers {
        let str = pe.all_data_to_string();
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
/// rusqlite connection
/// id of the Person to show
///
pub fn return_one_person(conn: &RQLConn, id: i32) -> Result<Person, SQError> {

    let mut stmt = conn.prepare("select persons.id,
                                                      persons.nom,
                                                      persons.prenom
                                               from persons
                                               where person.id = :id;")?;
    let pers = stmt.query_row_named(&[(":id", &id)],
                                             |row|
                                                 Ok(Person{
                                                    id:row.get(0)?,
                                                    nom: row.get(1)?,
                                                    prenom: row.get(2)?,
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
/// rusqlite connection,
/// Person to add,
///
pub fn add_person(conn: &RQLConn, pers : Person) -> Result<(), SQError> {

    conn.execute("insert into persons(nom, prenom) values (?1, ?2);",
                 params![pers.nom, pers.prenom],)?;
    println!("Nouvelle personne créée");
    Ok(())
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
pub fn modify_person(conn: &RQLConn, pers_id: i32, new_pers: Person) -> Result<(), SQError> {
    conn.execute("UPDATE persons SET nom = ?1, prenom = ?2 WHERE id = ?3;", params![new_pers.nom, new_pers.prenom, pers_id])?;
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
pub fn delete_person(conn: &RQLConn, pers_id: i32) -> Result<(), SQError> {
   unimplemented!()
}
