mod db_access;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use db_access::{DbConnection, DbPrivilege, InsertingPerson, Person};
use serde_derive::{Deserialize, Serialize};

struct AppState {
    db: Mutex<db_access::DbConnection>,
}

fn check_credentials(
    auth: BasicAuth,
    data: &web::Data<AppState>,
    required_privilege: DbPrivilege,
) -> Result<Vec<DbPrivilege>, String> {
    let db_conn = &data.db.lock().unwrap();
    if let Some(user) = db_conn.get_user_by_username(auth.user_id()) {
        if auth.password().is_some() && &user.password == auth.password().unwrap() {
            if user.privileges.contains(&required_privilege) {
                Ok(user.privileges.clone())
            } else {
                Err(format!(
                    "Insufficient privileges for user \"{}\".",
                    user.username
                ))
            }
        } else {
            Err(format!("Invalid password for user \"{}\".", user.username))
        }
    } else {
        Err(format!("User \"{}\" not found.", auth.user_id()))
    }
}

#[derive(Serialize)]
enum AuthenticationResult {
    LoggedUser(db_access::User),
    ErrorMessage(String),
}

#[get("/authenticate")]
async fn authenticate(auth: BasicAuth, data: web::Data<AppState>) -> impl Responder {
    println!("=== authenticate() ===");
    let db_conn = &data.db.lock().unwrap();
    if let Some(user) = db_conn.get_user_by_username(auth.user_id()) {
        if auth.password().is_some() && &user.password == auth.password().unwrap() {
            HttpResponse::Ok().json(AuthenticationResult::LoggedUser(user))
        } else {
            HttpResponse::Forbidden().json(AuthenticationResult::ErrorMessage(
                format!("Invalid password for user \"{}\".", user.username).to_string(),
            ))
        }
    } else {
        HttpResponse::Forbidden().json(AuthenticationResult::ErrorMessage(
            format!("User \"{}\" not found.", auth.user_id()).to_string(),
        ))
    }
}

#[get("/person/{id}")]
async fn get_person_by_id(
    auth: BasicAuth,
    data: web::Data<AppState>,
    info: web::Path<(u32,)>,
) -> impl Responder {
    println!("=== get_person_by_id() ===");
    match check_credentials(auth, &data, DbPrivilege::CanRead) {
        Ok(_) => {
            let id = info.0;
            let db_conn = &data.db.lock().unwrap();
            if let Some(person) = db_conn.get_person_by_id(id) {
                HttpResponse::Ok().json(person)
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(msg) => HttpResponse::Forbidden().json(&msg),
    }
}

#[derive(Deserialize)]
pub struct Filter {
    partial_name: Option<String>,
}

#[get("/persons")]
async fn get_persons(
    auth: BasicAuth,
    data: web::Data<AppState>,
    query: web::Query<Filter>,
) -> impl Responder {
    println!("=== get_persons() ===");
    match check_credentials(auth, &data, DbPrivilege::CanRead) {
        Ok(_) => {
            let db_conn = &data.db.lock().unwrap();
            let partial_name = &query.partial_name.clone().unwrap_or("".to_string());
            let persons = db_conn
                .get_persons_by_partial_name(partial_name)
                .collect::<Vec<_>>();
            HttpResponse::Ok().json(persons)
        }
        Err(msg) => HttpResponse::Forbidden().json(&msg),
    }
}

#[derive(Deserialize)]
pub struct ToDelete {
    id_list: Option<String>,
}

#[delete("/persons")]
async fn delete_persons(
    auth: BasicAuth,
    data: web::Data<AppState>,
    query: web::Query<ToDelete>,
) -> impl Responder {
    println!("=== delete_persons() ===");
    match check_credentials(auth, &data, DbPrivilege::CanWrite) {
        Ok(_) => {
            let db_conn = &mut data.db.lock().unwrap();
            let mut deleted_count = 0;
            query
                .id_list
                .clone()
                .unwrap_or("".to_string())
                .split_terminator(',')
                .for_each(|id| {
                    deleted_count += if db_conn.delete_by_id(id.parse::<u32>().unwrap()) {
                        1
                    } else {
                        0
                    };
                });
            HttpResponse::Ok().json(deleted_count)
        }
        Err(msg) => HttpResponse::Forbidden().json(&msg),
    }
}

#[post("/one_person")]
async fn insert_person(
    auth: BasicAuth,
    data: web::Data<AppState>,
    person: web::Json<InsertingPerson>,
) -> impl Responder {
    println!("=== insert_person() ===");
    match check_credentials(auth, &data, DbPrivilege::CanWrite) {
        Ok(_) => {
            let db_conn = &mut data.db.lock().unwrap();
            let new_id = db_conn.insert_person(person.into_inner());
            HttpResponse::Ok().json(new_id)
        }
        Err(msg) => HttpResponse::Forbidden().json(&msg),
    }
}

#[put("/one_person")]
async fn update_person(
    auth: BasicAuth,
    data: web::Data<AppState>,
    person: web::Json<Person>,
) -> impl Responder {
    println!("=== update_person() ===");
    match check_credentials(auth, &data, DbPrivilege::CanWrite) {
        Ok(_) => {
            let db_conn = &mut data.db.lock().unwrap();
            let person = person.into_inner();
            println!("updating person: {:?}", person);
            let updated = db_conn.update_person(person);
            HttpResponse::Ok().json(updated)
        }
        Err(msg) => HttpResponse::Forbidden().json(&msg),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allowed_origin("http://127.0.0.1:8080"))
            .app_data(Config::default().realm("PersonsApp"))
            .app_data(web::Data::new(AppState {
                db: Mutex::new(db_access::DbConnection::new()),
            }))
            .service(get_person_by_id)
            .service(authenticate)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
