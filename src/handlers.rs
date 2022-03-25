use crate::models::Status;
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::db;


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {status: "Ok".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connection to the database");

    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)> ) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connection to the database");

    let result = db::get_items(&client, path.0).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}