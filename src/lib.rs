pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewTodo, Todo};

pub fn create_todo<'a>(
        conn: &PgConnection,
        title: &'a str,
        content: &'a str,
) -> Result<Todo, Error> {
        use schema::todos;

        let new_todo = NewTodo {
                title: title,
                content: content,
        };

        diesel::insert_into(todos::table)
                .values(&new_todo)
                .get_result(conn)
}

pub fn get_todo_all<'a>(conn: &PgConnection) -> Result<Vec<Todo>, Error> {
        use schema::todos::dsl::*;

        todos.load::<Todo>(conn)
}

pub fn get_todo<'a>(conn: &PgConnection, todo_title: &'a str) -> Result<Vec<Todo>, Error> {
        use schema::todos::dsl::*;

        todos.filter(title.eq(todo_title)).load::<Todo>(conn)
}

pub fn delete_todo<'a>(conn: &PgConnection, todo_id: i32) -> Result<usize, Error> {
        use schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(todo_id))).execute(conn)
}
