use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::Path,
};
use serde::{Deserialize, Serialize};
use log;

use crate::todo::{Todo, self};

pub fn get_app() -> Router<> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/todo", get(get_todos))
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
// async fn get_todos(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}