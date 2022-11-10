
use actix_web::{get, post, web, Responder};
use std::sync::Mutex;
use serde::Deserialize;
use mysql::PooledConn;

mod bdd;

pub struct AppData {
    pub conn: Mutex<mysql::PooledConn>,
}

#[derive(Deserialize)]
pub struct NoteFormData {
    pub id: String,
    pub note: String,
}

#[post("/api/create")]
pub async fn create_note(info: web::Form<NoteFormData>, data: web::Data<AppData>) -> impl Responder {
    match data.conn.lock() {
        Ok(mut conn) => {
            let mut info_id: String = info.id.clone();
            if info_id.len() == 0 {
                info_id = bdd::notes::get_free_id(&mut conn);
            }
            match bdd::notes::insert_note(&mut conn, &info_id, info.note.clone()) {
                Ok(_) => format!("Success:{}", info_id),
                Err(e) => format!("Error: {}", e)
            }
        },
        Err(_) => format!("Error: cannot connect to database")
    }
}

#[get("/api/note/{note_id}")]
pub async fn get_note(note_id: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    match data.conn.lock() {
        Ok(mut conn) => {
            match bdd::notes::get_note(&mut conn, note_id.to_string()) {
                Ok(note) => format!("{}", note.note),
                Err(_) => format!("")
            }
        },
        Err(_) => format!("Error: cannot connect to database")
    }
}

pub fn get_db_conn() -> PooledConn {
    bdd::notes::get_db_conn()
}