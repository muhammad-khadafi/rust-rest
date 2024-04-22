use actix_web::{web, HttpResponse, Responder};
use crate::models::Book;
use crate::state::AppState;
use serde_json::json;

pub fn config_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/books", web::post().to(create_book))
       .route("/books/{id}", web::get().to(get_book))
       .route("/books/{id}", web::put().to(update_book))
       .route("/books/{id}", web::delete().to(delete_book));
}

async fn create_book(data: web::Data<AppState>, book: web::Json<Book>) -> impl Responder {
    data.create_book(book.into_inner());
    HttpResponse::Created().finish()
}

async fn get_book(data: web::Data<AppState>, book_id: web::Path<u32>) -> impl Responder {
    if let Some(book) = data.get_book(*book_id) {
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_book(data: web::Data<AppState>, book_id: web::Path<u32>, book: web::Json<Book>) -> impl Responder {
    if let Some(book) = data.update_book(*book_id, book.into_inner()) {
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_book(data: web::Data<AppState>, book_id: web::Path<u32>) -> impl Responder {
    if data.delete_book(*book_id) {
        HttpResponse::Ok().json(json!({"message": "Book deleted"}))
    } else {
        HttpResponse::NotFound().finish()
    }
}
