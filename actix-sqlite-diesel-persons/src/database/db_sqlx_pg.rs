//src/database/db_sqlx.rs

use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow, Row};
use sqlx::postgres::PgRow;
use anyhow::Result as anyResult;

use crate::models::person::*;


// implementation of Actix Responder for Person struct so we can return Person from action handler
impl Responder for Person {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

// Implementation for Person struct, functions for read/write/update and delete Person from database
impl Person {
    pub async fn find_all(pool: &PgPool) -> anyResult<Vec<Person>> {
        let mut persons = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, first_name, last_name
                    FROM persons
                ORDER BY id
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            persons.push(Person {
                id: rec.id,
                first_name: rec.first_name,
                last_name: rec.last_name
            });
        }

        Ok(persons)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> anyResult<Person> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM persons WHERE id = $1
                "#,
                id
            )
            .fetch_one(&*pool)
            .await?;

        Ok(Person {
            id: rec.id,
            first_name: rec.first_name,
            last_name: rec.last_name
        })
    }

    pub async fn create(person: InsertablePerson, pool: &PgPool) -> anyResult<Person> {
        let mut tx = pool.begin().await?;
        let pers = sqlx::query("INSERT INTO persons (first_name, last_name) VALUES ($1, $2) RETURNING id, first_name, last_name")
            .bind(&person.first_name)
            .bind(person.last_name)
            .map(|row: PgRow| {
                Person {
                    id: row.get(0),
                    first_name: row.get(1),
                    last_name: row.get(2)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(pers)
    }

    pub async fn update(id: i32, person: InsertablePerson, pool: &PgPool) -> anyResult<Person> {
        let mut tx = pool.begin().await.unwrap();
        let pers = sqlx::query("UPDATE persons SET first_name = $1, last_name = $2 WHERE id = $3 RETURNING id, first_name, last_name")
            .bind(&person.first_name)
            .bind(person.last_name)
            .bind(id)
            .map(|row: PgRow| {
                Person {
                    id: row.get(0),
                    first_name: row.get(1),
                    last_name: row.get(2)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(pers)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> anyResult<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM persons WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted)
    }
}