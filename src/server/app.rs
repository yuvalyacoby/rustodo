use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::Path,
};
use serde::{Deserialize};
use log;

use crate::{todo::{Todo, self}, status::Status};

pub fn get_app() -> Router<> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/todo", get(get_todos).post(update_todo))
        .route("/todo/:todo_name", get(get_todo));

    return app;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_todos() -> (StatusCode, Json<Vec<Todo>>) {
    log::debug!("get_todos called");
    let todos = todo::get_todos().unwrap();
    (StatusCode::ACCEPTED, Json(todos))
}

async fn get_todo(Path(todo_name): Path<String>) -> (StatusCode, Json<Todo>) {
    log::debug!("get_todo called with {:?}", todo_name);
    let todo = todo::get_todo(&todo_name).unwrap();
    (StatusCode::ACCEPTED, Json(todo))
}

async fn update_todo(
    Json(payload): Json<UpdateTodoRequest>,
) -> (StatusCode, Json<Todo>) {
    log::debug!("update_todo called with {:?}", payload);
    let todo = todo::update_todo(payload.name, payload.status, payload.description).unwrap();

    (StatusCode::CREATED, Json(todo))
}



#[derive(Deserialize, Debug)]
struct UpdateTodoRequest {
    name: String,
    description: Option<String>,
    status: Option<Status>
}