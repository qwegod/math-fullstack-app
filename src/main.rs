mod task;

use actix_files::Files;
use actix_session::{storage::CookieSessionStore, Session, SessionExt, SessionMiddleware};
use actix_web::{
    cookie::Key,
    dev::{Service, ServiceResponse},
    get, middleware,
    middleware::Logger,
    post, web,
    web::Path,
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

struct Rights {
    admin: Arc<Mutex<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Book {
    name: String,
    year: usize,
    id: usize,
}

#[derive(Deserialize)]
struct Admin {
    login: String,
    password: String,
}

#[derive(Clone, Debug)]
struct Shelf {
    contain: Vec<Book>,
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

async fn admin_panel() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../templates/admin-panel.html"))
}

async fn handle_404() -> impl Responder {
    HttpResponse::NotFound().body(include_str!("../templates/not-found.html"))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../templates/index.html"))
}

#[get("/admin")]
async fn admin(admin: Query<Admin>, session: Session) -> impl Responder {
    if admin.login == "login" && admin.password == "1111" {
        session.insert("authenticated", &true).unwrap();
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

#[get("/p/{first_p}/{second_p}")]
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

#[post("/http-version")]
async fn http(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(format!("HTTP version: {:?}", req.version()))
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
            .wrap(middleware::NormalizePath::trim())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(Data::new(rights_clone.clone()))
            .service(admin)
            .service(index)
            .service(login)
            .service(no_rights)
            .service(take_file)
            .service(http)
            .service(
                scope("/task")
                    .app_data(Data::clone(&tasks_clone))
                    .service(resource("/create").route(web::post().to(task::create)))
                    .service(resource("/list").route(web::post().to(task::list)))
                    .service(resource("/remove").route(web::post().to(task::remove)))
                    .service(resource("/start").route(web::post().to(task::start)))
                    .service(resource("/instruction").route(web::get().to(task::instruction))),
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
                        if let Some(true) = req
                            .get_session()
                            .get::<bool>("authenticated")
                            .unwrap_or(None)
                        {
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
            .default_service(route().to(handle_404))
    })
    .shutdown_timeout(10)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
