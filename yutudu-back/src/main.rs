use std::ops::Deref;

use actix_web::{App, HttpResponse, HttpServer, Responder, ResponseError, get, http::{self, StatusCode}, post, web};
use yutudu_common::{TodoAction};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum WebError {
    #[error("Request failed: {0}")]
    Custom(String),
    #[error("Something unexpected went wrong.")]
    Other
}

impl ResponseError for WebError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            .set_body(self.to_string().into())
    }
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     "Hello world!"
// }

#[post("/api/todo")]
async fn todo(req_body: web::Json<TodoAction>) -> impl Responder {
    match req_body.deref() {
        TodoAction::AddTask(name) => {
            println!("Add task {}", name);
            Ok("Created task")
        }
        TodoAction::CompleteTask(id) => {
            println!("Complete task {}", id);
            Ok("Completed task")
        }
        TodoAction::DeleteTask(id) => {
            println!("Delete task {}", id);
            Ok("Deleted task")
        }
        TodoAction::ReopenTask(id) => {
            Err(WebError::Other)
            //println!("Reopen task {}", id);
        }
    }
}

#[get("/api/todo/{name}")]
async fn new_todo(web::Path(name): web::Path<String>) -> impl Responder {
    serde_json::to_string_pretty(&TodoAction::AddTask(name)).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(todo).service(new_todo)
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
