use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::{AppState, Todo};
use super::models::{CreateTodo, UpdateTodo};

#[get("/todolist/todos")]
async fn get_todos(app_state: web::Data<AppState>) -> impl Responder {
    let todos = app_state.todos.lock().unwrap();
    HttpResponse::Ok().json(todos.clone())
}

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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_todos);
    cfg.service(create_todo);
    cfg.service(update_todo);
    cfg.service(delete_todo);
}