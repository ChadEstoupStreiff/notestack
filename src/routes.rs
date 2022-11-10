use actix_web::{get, Result};
use actix_files::NamedFile;
use std::io::Error;

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