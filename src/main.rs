use actix_web::{web, App, HttpServer};
mod handlers;
mod models;
mod state;

use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(handlers::config_handlers)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
