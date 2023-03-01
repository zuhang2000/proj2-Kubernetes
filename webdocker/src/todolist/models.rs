use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateTodo {
    pub date: i64,
    pub completed: bool,
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateTodo {
    pub completed: bool,
    pub title: String,
}