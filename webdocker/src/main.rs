use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::io;



// module models
mod models {
    use serde::Deserialize;

    // createTodo module
    #[derive(Deserialize, Clone)]
    pub struct CreateTodo {
        pub date: i64,
        pub completed: bool,
        pub title: String,
    }

    // updateTodo module
    #[derive(Deserialize, Clone)]
    pub struct UpdateTodo {
        pub completed: bool,
        pub title: String,
    }
}


// module services
mod services {
    use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
    use crate::{AppState, Todo};
    use super::models::{CreateTodo, UpdateTodo};

    // retrieve all todo tasks here
    #[get("/todolist/todos")]
    async fn get_todos(app_state: web::Data<AppState>) -> impl Responder {
        let todos = app_state.todos.lock().unwrap();
        HttpResponse::Ok().json(todos.clone())
    }


    // create one todo task
    #[post("/todolist/todos")]
    async fn create_todo(app_state: web::Data<AppState>, passin: web::Json<CreateTodo>) -> impl Responder {
        let mut todos = app_state.todos.lock().unwrap();
        let mut id: u32 = 0;
        for i in todos.iter() {
            if i.id > id {
                id = i.id;
            }
        }

        todos.push(Todo {
            id: id + 1,
            date: passin.date,
            completed: passin.completed,
            title: passin.title.clone(),
        });

        HttpResponse::Ok().json(todos.to_vec())
    }


    //update an existing todo asks given id
    #[put("/todolist/todos/{id}")]
    async fn update_todo(app_state: web::Data<AppState>, id: web::Path<u32>, passin: web::Json<UpdateTodo>) -> impl Responder {
        let mut todos = app_state.todos.lock().unwrap();
        let mut index: i32 = -1;
        for i in 0..todos.len() {
            if todos[i].id == *id {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            return HttpResponse::NotFound().body("Todo not found");
        }

        todos[index as usize].completed = passin.completed;
        todos[index as usize].title = passin.title.clone();

        HttpResponse::Ok().json(todos.to_vec())
    }


    // delete an existing todo task given id
    #[delete("/todolist/todos/{id}")]
    async fn delete_todo(app_state: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
        let mut todos = app_state.todos.lock().unwrap();
        let mut index: i32 = -1;
        for i in 0..todos.len() {
            if todos[i].id == *id {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            return HttpResponse::NotFound().body("Todo not found");
        }

        todos.remove(index as usize);

        HttpResponse::Ok().json(todos.to_vec())
    }


    // configurations
    pub fn init_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(get_todos);
        cfg.service(create_todo);
        cfg.service(update_todo);
        cfg.service(delete_todo);
    }
}

//stores all todo tasks
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



// home page
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
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}