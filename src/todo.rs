use actix_web::{HttpResponse, Responder, web, get, post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: Option<u32>,
    content: String,
    done: bool,
}

#[get("/todos/{id}")]
pub async fn get_todo(web::Path(id): web::Path<u32>) -> impl Responder {
    println!("get todo");
    let id_option: Option<u32> = Some(id);
    HttpResponse::Ok().json(Todo {
        id: id_option,
        content: String::from("Todo"),
        done: false,
    })
}

#[post("/todo")]
pub async fn post_todo(todo: web::Json<Todo>) -> impl Responder {
    println!("post todo");
    println!("{:?}", todo);
    HttpResponse::Ok().body("ok")
}