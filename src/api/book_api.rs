use crate::{models::books_model::Book, repository::mongodb_repo::MongoRepo};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/books")]
pub async fn create_book(db: Data<MongoRepo>, book: Json<Book>) -> HttpResponse {
    let book = Book {
        id: None,
        title: book.title.to_owned(),
        message: book.message.to_owned(),
        owner_id: book.owner_id.to_owned(),
    };

    let data = db.create_book(book).await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get["/book"]]
pub async fn get_book(db: Data<MongoRepo>, names: String) -> HttpResponse {
    // let data = db.get_books.

    let data = db.get_book(names).await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/books")]
pub async fn get_books(db: Data<MongoRepo>) -> HttpResponse {
    let data = db.get_books().await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
