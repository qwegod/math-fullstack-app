mod task;

use actix_files::Files;
use actix_web::web::{Json, Path};
use actix_web::{
    dev::{Service, ServiceResponse},
    get,
    middleware::Logger,
    web,
    web::{resource, route, scope, Data, Query},
    App, Either, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::FutureExt;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::{
    env,
    io::Result,
    sync::{Arc, Mutex},
};

#[derive(Serialize, Clone, Debug, Deserialize)]
struct Book {
    name: String,
    year: usize,
    id: usize,
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

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

struct Rights {
    admin: Arc<Mutex<bool>>,
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

    fn add(data: Query<HashMap<String, String>>, shelf: Data<Mutex<Shelf>>) {
        let mut shelf = shelf.lock().unwrap();
        let id = shelf.contain.len();
        let year;
        match data
            .clone()
            .into_inner()
            .get("year")
            .unwrap()
            .parse::<usize>()
        {
            Ok(o) => year = o,
            Err(e) => {
                error!("{}", e);
                return;
            }
        }
        shelf.contain.push(Book {
            id,
            name: data.clone().into_inner().get("name").unwrap().clone(),
            year,
        });
    }
}

async fn book(data: Query<HashMap<String, String>>, shelf: Data<Mutex<Shelf>>) -> impl Responder {
    Shelf::add(data, shelf);
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../templates/index.html"))
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

#[get("/login")]
async fn login(rights: Data<Arc<Rights>>) -> impl Responder {
    if *rights.admin.lock().unwrap() == true {
        HttpResponse::Ok().body(include_str!("../templates/admin-panel.html"))
    } else {
        HttpResponse::Ok().body(include_str!("../templates/login.html"))
    }
}

#[get("/no-rights")]
async fn no_rights() -> impl Responder {
    HttpResponse::Unauthorized().body(include_str!("../templates/no-rights.html"))
}

#[get("/p/{query_one}/{query_two}")]
async fn take_file(path: Path<(String, String)>) -> Either<Vec<u8>, impl Responder> {
    let (qwe, zxc) = (&path.0, &path.1);

    match std::fs::File::open(format!("{}/{}", qwe, zxc)) {
        Ok(mut res) => {
            let mut contents = Vec::new();
            res.read_to_end(&mut contents)
                .expect("Failed to read image file");
            Either::Left(contents)
        }
        Err(e) => {
            error!("{}", e);
            Either::Right(HttpResponse::BadRequest().finish())
        }
    }
}

async fn http(req: HttpRequest) -> impl Responder {
    info!("HTTP version: {:?}", req.version());
    HttpResponse::Ok().append_header(("Location", "/")).finish()
}

async fn settings() -> String {
    "Settings".to_string()
}

async fn submit(data: Json<User>) -> impl Responder {
    format!("Hello, {}!", data.name)
}

async fn admin_panel() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../templates/admin-panel.html"))
}

async fn get_html(data: web::Form<User>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Submitted username: {} password: {}",
        data.name, data.password
    ))
}

async fn test_page() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn handle_404() -> impl Responder {
    HttpResponse::NotFound().body(include_str!("../templates/not-found.html"))
}

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let shelf = Data::new(Mutex::new(Shelf::new()));
    let tasks = Data::new(Mutex::new(task::Tasks::new()));
    let rights = Arc::new(Rights {
        admin: Arc::new(Mutex::new(false)),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        let shelf_service_clone = Data::clone(&shelf);
        let rights_clone = Arc::clone(&rights);
        let tasks_clone = Data::clone(&tasks);
        App::new()
            .wrap(logger)
            .app_data(Data::new(rights_clone.clone()))
            .service(admin)
            .service(index)
            .service(login)
            .service(no_rights)
            .service(take_file)
            .service(
                scope("/task")
                    .app_data(Data::clone(&tasks_clone))
                    .service(resource("/create").route(web::post().to(task::create)))
                    .service(resource("/list").route(web::post().to(task::list)))
                    .service(resource("/remove").route(web::post().to(task::remove)))
                    .service(resource("/start").route(web::post().to(task::start)))
                    .service(
                        resource("/finish_simulation")
                            .route(web::post().to(task::finish_simulation)),
                    )
                    .service(
                        resource("/error_simulation").route(web::post().to(task::error_simulation)),
                    ),
            )
            .service(
                scope("/book")
                    .app_data(Data::clone(&shelf_service_clone))
                    .service(resource("/{tail:.*}").route(web::get().to(book))),
            )
            .service(
                scope("/p")
                    .wrap_fn(move |req, srv| {
                        if !req.path().split('/').last().unwrap().contains(".") {
                            let res = HttpResponse::Found()
                                .append_header(("Location", "/no-rights"))
                                .finish();
                            let (parts, _) = req.into_parts();
                            let service_response = ServiceResponse::new(parts, res);
                            async { Ok(service_response) }.boxed_local()
                        } else {
                            srv.call(req)
                        }
                    })
                    .service(Files::new("/", "./").show_files_listing()),
            )
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
            .service(scope("/shelf").route(
                "/list",
                web::get().to(move || {
                    let shelf = Data::clone(&shelf_service_clone);
                    info!("LIST SHELF: {:?}", shelf);
                    async move {
                        let list = Shelf::list(&shelf);
                        HttpResponse::Ok().json(list)
                    }
                }),
            ))
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
