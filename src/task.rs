use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    id: usize,
    status: String,
    name: String,
    data: String,
}

#[derive(Deserialize)]
pub struct TaskSetup {
    name: String,
    data: String,
}

#[derive(Deserialize)]
pub struct TaskId {
    id: usize,
}

#[derive(Serialize)]
pub struct Tasks {
    list: Mutex<Vec<Task>>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks {
            list: Mutex::new(Vec::new()),
        }
    }
}

pub async fn create(tasks: Data<Mutex<Tasks>>, task: Json<TaskSetup>) -> impl Responder {
    let mut tasks_lock = tasks.lock().unwrap();
    let mut tasks_list_lock = tasks_lock.list.lock().unwrap();

    let id = tasks_list_lock.len();

    let mut new_task = task.into_inner();

    tasks_list_lock.push(Task {
        id,
        status: "pending".to_string(),
        name: new_task.name,
        data: new_task.data,
    });

    HttpResponse::Ok().finish()
}

pub async fn list(tasks: Data<Mutex<Tasks>>) -> impl Responder {
    let tasks_data = tasks.lock().unwrap();

    let tasks_list = tasks_data.list.lock().unwrap().clone();

    HttpResponse::Ok().json(tasks_list.clone())
}

pub async fn remove(tasks: Data<Mutex<Tasks>>, task: Json<TaskId>) -> impl Responder {
    let mut tasks_lock = tasks.lock().unwrap();
    let mut tasks_list_lock = tasks_lock.list.lock().unwrap();

    tasks_list_lock.remove(task.id);

    HttpResponse::Ok().finish()
}

pub async fn start(tasks: Data<Mutex<Tasks>>, task: Json<TaskId>) -> impl Responder {
    let mut tasks_lock = tasks.lock().unwrap();
    let mut tasks_list_lock = tasks_lock.list.lock().unwrap();

    tasks_list_lock[task.id].status = "solving".to_string();

    HttpResponse::Ok().finish()
}

pub async fn finish_simulation(tasks: Data<Mutex<Tasks>>, task: Json<TaskId>) -> impl Responder {
    let mut tasks_lock = tasks.lock().unwrap();
    let mut tasks_list_lock = tasks_lock.list.lock().unwrap();

    tasks_list_lock[task.id].status = "OK".to_string();

    HttpResponse::Ok().finish()
}

pub async fn error_simulation(tasks: Data<Mutex<Tasks>>, task: Json<TaskId>) -> impl Responder {
    let mut tasks_lock = tasks.lock().unwrap();
    let mut tasks_list_lock = tasks_lock.list.lock().unwrap();

    tasks_list_lock[task.id].status = "Err".to_string();

    HttpResponse::Ok().finish()
}
