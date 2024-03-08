use actix_files::Files;
use actix_web::{
    get,
    web,
    middleware::Logger,
    dev::{Service, ServiceResponse},
    web::{resource, route, scope, Data, Query},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::FutureExt;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::Result,
    sync::{Arc, Mutex},
};

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
        let shelf = data.lock().unwrap();
        shelf.contain.clone()
    }

    fn add(data: &Mutex<Shelf>) {
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
    let content = fs::read_to_string("./templates/index.html").unwrap();
    HttpResponse::Ok().body(content)
}

#[get("/admin")]
async fn admin(admin: Query<Admin>, rights: Data<Arc<Rights>>) -> impl Responder {
    *rights.admin.lock().unwrap() = true;
    if format!("{}", admin.login) == "login" && format!("{}", admin.password) == "1111" {
        HttpResponse::Found()
            .append_header(("Location", "/admin-panel"))
            .finish()
    } else {
        error!("Incorrect login attempt");
        HttpResponse::Unauthorized().body("Incorrect login or password")
    }
}


#[get("/no-rights")]
async fn no_rights() -> impl Responder {
    HttpResponse::Unauthorized().body("no rights")
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
    admin: Arc<Mutex<bool>>,
}

async fn submit(data: web::Json<User>) -> impl Responder {
    format!("Hello, {}!", data.name)
}


async fn admin_panel() -> impl Responder {
    let content = fs::read_to_string("./templates/admin-panel.html").unwrap();
    HttpResponse::Ok().body(content)
}

async fn get_html(data: web::Form<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Submitted username: {}", data.name))
}

async fn test_page() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn handle_404() -> impl Responder {
    let content = fs::read_to_string("./templates/not_found.html").unwrap();
    HttpResponse::NotFound().body(content)
}

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let shelf = Arc::new(Mutex::new(Shelf::new()));
    let rights = Arc::new(Rights {
        admin: Arc::new(Mutex::new(false)),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        let shelf_service_clone = Arc::clone(&shelf);
        let rights_clone = Arc::clone(&rights);
        App::new()
            .wrap(logger)
            .app_data(Data::new(rights_clone.clone()))
            .service(admin)
            .service(index)
            .service(no_rights)
            .service(
                scope("/admin-panel")
                    .wrap_fn(move |req, srv| {
                        if *rights_clone.admin.lock().unwrap() {
                            srv.call(req)
                        } else {
                            let res = HttpResponse::Found()
                                .append_header(("Location", "/no-rights"))
                                .finish();
                            let (parts, _) = req.into_parts();
                            let service_response = ServiceResponse::new(parts, res);
                            async { Ok(service_response) }.boxed_local()
                        }
                    })
                    .service(resource("").to(admin_panel))
                    .service(resource("/settings").to(settings))
                    .service(Files::new("/dir", "./").show_files_listing()),
            )
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
            .route("/http", web::get().to(http))
            .route("/submit", web::post().to(submit))
            .route("/html", web::post().to(get_html))
            .route("/test_page", web::get().to(test_page))
            .default_service(route().to(handle_404))
    })
    .shutdown_timeout(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
