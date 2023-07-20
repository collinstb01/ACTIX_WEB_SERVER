mod api;
mod middleware;
mod models;
mod repository;

//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, delete_user, get_user, get_users, update_user};
// use middleware::user_middleware::middleware_handler;
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: MongoRepo = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            // // Register the custom middleware before other routes
            // .wrap(middleware_handler)
            // // Optional: Use actix-web Logger middleware to log requests
            // .wrap(Logger::default())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
