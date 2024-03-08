use actix_web::{get, guard::Host, http, middleware::Logger, post, web, web::{resource, route, scope, Data, Query}, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, Either};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::Result,
    sync::{Arc, Mutex},
};
use std::panic::Location;
use std::time::Duration;
use actix_web::http::header::LOCATION;
use actix_web::web::{get, redirect, Redirect};

#[derive(Serialize, Clone, Debug)]
struct Book {
    id: usize,
    name: String,
    year: usize,
}

#[derive(Clone, Debug)]
struct Shelf {
    contain: Vec<Book>,
}

#[derive(Deserialize)]
struct Admin {
    login: String,
    password: String,
}

impl Shelf {
    fn new() -> Self {
        info!("Shelf created");
        Shelf {
            contain: Vec::new(),
        }
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
    let content = fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok().body(content)
}



#[get("/admin")]
async fn admin(admin: Query<Admin>, rights: Data<Arc<Rights>>) -> impl Responder {
    *rights.admin.lock().unwrap() = true;
    info!("rights = true");
    if format!("{}", admin.login) == "login" && format!("{}", admin.password) == "1111" {
        info!("red");
        HttpResponse::Found().append_header(("Location", "/admin-panel")).finish()
    } else {
        error!("Incorrect login attempt");
        HttpResponse::Unauthorized().body("Incorrect login or password")
    }
}




async fn http(req: HttpRequest) -> impl Responder {
    info!("HTTP version: {:?}", req.version());
    HttpResponse::Ok().finish()
}

async fn settings() -> String {
    "Settings".to_string()
}

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

struct Rights {
    admin: Arc<Mutex<bool>>
}

async fn submit(data: web::Json<User>) -> impl Responder {
    format!("Hello, {}!", data.name)
}




#[get("/no-rights")]
async fn no_rights() -> impl Responder {
    info!("please login");
    HttpResponse::Unauthorized().body("no rights")
}




#[get("/admin-panel")]
async fn admin_panel(rights: Data<Arc<Rights>>) -> Either<impl Responder, impl Responder> {
    let admin_lock = rights.admin.lock().unwrap();
    if *admin_lock {
        info!("r");
        info!("Visited admin panel");
        Either::Left(HttpResponse::Ok().body("Admin panel content"))
    } else {
        info!("no r");
        info!("redirect");
        Either::Right(HttpResponse::Found()
            .append_header(("Location", "/no-rights"))
            .finish())
    }
}




async fn get_html(data: web::Form<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Submitted username: {}", data.name))
}

async fn test_page() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn handle_404() -> impl Responder {
    HttpResponse::NotFound().body(" <style> body { background-color: black } h1 { color: white } </style> <h1> Not Found </h1>")
}

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let shelf = Arc::new(Mutex::new(Shelf::new()));
    let rights = Arc::new(Rights { admin: Arc::new(Mutex::new(false)) });

    HttpServer::new(move || {
        let logger = Logger::default();
        let shelf_service_clone = Arc::clone(&shelf);
        let rights_clone = Arc::clone(&rights);
        App::new()
            .wrap(logger)
            .service(index)
            .service(admin)
            .service(no_rights)
            .app_data(Data::new(rights_clone))
            .service(scope("/admin").route("/settings", web::get().to(settings)))
            .service(admin_panel)
            .service(
                scope("/shelf")
                    .route(
                        "/list",
                        web::get().to({
                            let shelf_service_clone = Arc::clone(&shelf_service_clone);
                            move || {
                                let shelf = Arc::clone(&shelf_service_clone);
                                async move {
                                    let list = Shelf::list(&shelf);
                                    HttpResponse::Ok().json(list)
                                }
                            }
                        }),
                    )
                    .route(
                        "/add",
                        web::get().to({
                            let shelf_service_clone = Arc::clone(&shelf_service_clone);
                            move || {
                                let shelf = shelf_service_clone.clone();
                                async move {
                                    Shelf::add(&shelf);
                                    HttpResponse::Ok().finish()
                                }
                            }
                        }),
                    ),
            )
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
