use super::schema::todos;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub done: bool,
}
