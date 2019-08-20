#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate diesel_todo_server;

use self::diesel_todo_server::*;
use self::models::Todo;
use diesel::pg::PgConnection;
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::error::{ApiError, ApiResponse};

mod error;

#[derive(Serialize, Deserialize)]
struct TodoPost {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct TodosJson {
    todos: Vec<Todo>,
}

struct AppState {
    connection: Mutex<PgConnection>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, rusty rocket fuelled with diesel!"
}

#[get("/api/list?<query>")]
fn return_list(query: Option<&RawStr>, state: State<AppState>) -> ApiResponse<String> {
    let conn = state.connection.lock().unwrap();
    match &query {
        None => match get_todo_all(&conn) {
            Ok(res) => {
                let res_string = serde_json::to_string(&TodosJson { todos: res }).unwrap();
                Ok(format!("{}\n", res_string))
            }
            Err(_err) => Err(ApiError::NotFound),
        },
        Some(query) => match get_todo(&conn, query) {
            Ok(res) => {
                let res_string = serde_json::to_string(&TodosJson { todos: res }).unwrap();
                Ok(format!("{}\n", res_string))
            }
            Err(_err) => Err(ApiError::NotFound),
        },
    }
}

#[post("/api/post", format = "application/json", data = "<todo>")]
fn post_data<'r>(todo: Json<TodoPost>, state: State<'r, AppState>) -> ApiResponse<String> {
    let conn = state.connection.lock().unwrap();
    match create_todo(&conn, &todo.0.title, &todo.0.content) {
        Ok(_res) => Ok(format!("Successfully posted.\n")),
        Err(_err) => Err(ApiError::AlreadyExists),
    }
}

#[delete("/api/delete?<query>")]
fn delete_item(query: i32, state: State<AppState>) -> ApiResponse<String> {
    let conn = state.connection.lock().unwrap();
    match delete_todo(&conn, query) {
        Ok(_res) => Ok(format!("Success deleting.\n")),
        Err(_err) => Err(ApiError::FailedSaving),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, return_list, post_data, delete_item])
        .manage(AppState {
            connection: Mutex::new(establish_connection()),
        })
        .launch();
}

// PR
// Todo list
//  - GET list?query="string"
//  - POST data=string
//  - DELETE by query="string"
