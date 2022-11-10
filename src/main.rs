use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use actix_files::NamedFile;
use serde::Deserialize;
use std::sync::Mutex;
use std::io::Error;

mod bdd;
pub use crate::bdd::notes;


// BACK PART
#[derive(Deserialize)]
struct NoteFormData {
    id: String,
    note: String,
}


#[get("/")]
async fn hello() -> Result<NamedFile, Error> {
    match NamedFile::open("./web/index.html") {
        Ok(file) => Ok(file),
        Err(e) => Err(e)
    }
}

#[get("/create")]
async fn create_note_form() -> Result<NamedFile, Error> {
    match NamedFile::open("./web/form.html") {
        Ok(file) => Ok(file),
        Err(e) => Err(e)
    }
}


#[get("/note/{note_id}")]
async fn see_note() -> Result<NamedFile, Error> {
    match NamedFile::open("./web/note.html") {
        Ok(file) => Ok(file),
        Err(e) => Err(e)
    }
}


#[post("/api/create")]
async fn create_note(info: web::Form<NoteFormData>, data: web::Data<AppData>) -> impl Responder {
    match data.conn.lock() {
        Ok(mut conn) => {
            let mut info_id: String = info.id.clone();
            if info_id.len() == 0 {
                info_id = bdd::notes::get_free_id(&mut conn);
            }
            match notes::insert_note(&mut conn, &info_id, info.note.clone()) {
                Ok(_) => format!("Success:{}", info_id),
                Err(e) => format!("Error: {}", e)
            }
        },
        Err(_) => format!("Error: cannot connect to database")
    }
}

#[get("/api/note/{note_id}")]
async fn get_note(note_id: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    match data.conn.lock() {
        Ok(mut conn) => {
            match notes::get_note(&mut conn, note_id.to_string()) {
                Ok(note) => format!("{}", note.note),
                Err(_) => format!("")
            }
        },
        Err(_) => format!("Error: cannot connect to database")
    }
}

// App
struct AppData {
    conn: Mutex<mysql::PooledConn>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppData{
                conn: Mutex::new(notes::get_db_conn())
            }))
            .service(actix_files::Files::new("/assets", "./web/assets").show_files_listing())
            .service(hello)
            .service(create_note_form)
            .service(see_note)
            .service(create_note)
            .service(get_note)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}