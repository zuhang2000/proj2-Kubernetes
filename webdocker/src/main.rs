use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::io;

mod todolist;
use todolist::services;


struct AppState {
    todos: Mutex<Vec<Todo>>
}

#[derive(Deserialize, Serialize, Clone)]
struct Todo {
    id: u32,
    date: i64,
    completed: bool,
    title: String,
}


#[get("/")]
async fn index() -> String {
    "This is the home page".to_string()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app_data = web::Data::new(AppState {
        todos: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}