use actix_web::{get, guard::Host, web, web::{scope, Data}, App, middleware::Logger, HttpResponse, HttpServer, Responder, HttpRequest, http, post};
use actix_web::web::{Query, resource, route};
use log::{ info, warn, error, debug };
use std::{env, fs};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Clone, Debug)]
struct Book {
    id: usize,
    name: String,
    year: usize
}

#[derive(Clone, Debug)]
struct Shelf {
    contain: Vec<Book>
}


#[derive(Deserialize)]
struct Admin {
    login : String,
    password: String
}


impl Shelf {
    fn new() -> Self {
        info!("Shelf created");
        Shelf { contain: Vec::new() }
    }

    fn list(data: &Mutex<Shelf>) -> Vec<Book> {
        info!("Shelf list printed");
        let shelf = data.lock().unwrap();
        shelf.contain.clone()
    }



    fn add(data: &Mutex<Shelf>) {
        info!("Book added");
        let mut shelf = data.lock().unwrap();
        let id = shelf.contain.len();
        shelf.contain.push(Book {
            id,
            name: format!("Book {}", id),
            year: id,
        });
    }

}



#[get("/")]
async fn index() -> impl Responder {
    info!("Main page");
    let content = fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok().body(content)
}



#[get("/admin")]
async fn admin(admin: Query<Admin>) -> impl Responder {
    info!("Admin page");
    if format!("{}", admin.login) == "login" && format!("{}", admin.password) == "1111" {
        HttpResponse::Ok().body(r#" <style>
                    body {
                        background-color: black;
                    }
                    a {
                        color: white;
                    }
                </style> <a href = "/settings" > settings </a>"#)
    }
    else{
        error!("Incorrect login attempt");
        HttpResponse::Unauthorized().body("Incorrect login or password")
    }
}

async fn http(req: HttpRequest) -> impl Responder {
    info!("HTTP version: {:?}", req.version());
    HttpResponse::Ok().finish()
}

async fn settings() -> String {
    info!("Settings page");
    "Settings".to_string()
}

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}


async fn submit(data: web::Json<User>) -> impl Responder {
    format!("Hello, {}!", data.name)
}


async fn get_html(data: web::Form<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Submitted username: {}", data.name))
}


async fn test_page() -> impl Responder {
    "test"
}


async fn handle_404() -> impl Responder {
    HttpResponse::NotFound().body(" <style> body { background-color: black } h1 { color: white } </style> <h1> Not Found </h1>")

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let shelf = Arc::new(Mutex::new(Shelf::new()));

    HttpServer::new(move || {
        let logger = Logger::default();
        let shelf_service_clone = Arc::clone(&shelf);
        App::new()
            .wrap(logger)
            .service(index)
            .service(admin)
            .service(
                scope("/shelf")
                    .route("/list", web::get().to({
                        let shelf_service_clone = Arc::clone(&shelf_service_clone);
                        move || {
                            let shelf = Arc::clone(&shelf_service_clone);
                            async move {
                                let list = Shelf::list(&shelf);
                                HttpResponse::Ok().json(list)
                            }
                        }
                    }))
                    .route("/add", web::get().to({
                        let shelf_service_clone = Arc::clone(&shelf_service_clone);
                        move || {
                            let shelf = shelf_service_clone.clone();
                            async move {
                                Shelf::add(&shelf);
                                HttpResponse::Ok().finish()
                            }
                        }
                    }))
            )
            .service(scope("/admin").route("/settings", web::get().to(settings)))
            .default_service(route().to(handle_404))
            .route("/http", web::get().to(http))
            .route("/submit", web::post().to(submit))
            .route("/html", web::post().to(get_html))
            .route("/test_page", web::get().to(test_page))
    })
        .shutdown_timeout(10)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

