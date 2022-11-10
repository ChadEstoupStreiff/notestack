use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod routes;
mod api;

// API

// App
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(api::AppData{
                conn: Mutex::new(api::get_db_conn())
            }))
            .service(actix_files::Files::new("/assets", "./web/assets").show_files_listing())
            .service(routes::hello)
            .service(routes::create_note_form)
            .service(routes::see_note)
            .service(api::create_note)
            .service(api::get_note)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}