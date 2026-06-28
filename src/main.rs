mod config;
mod models;
mod handlers;
mod middleware;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use actix_files::Files;
use sqlx::mysql::MySqlPool;
use std::fs;
use redis::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = config::Config::from_env();
    
    fs::create_dir_all(&config.upload_dir).expect("Failed to create upload directory");

    // mysql连接池
    let pool = MySqlPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // redis链接
    let redis_client = Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");
    let redis_manager = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("Failed to connect to Redis");

    let server_host = config.server_host.clone();
    let server_port = config.server_port;
    let upload_dir = config.upload_dir.clone();

    log::info!("Starting server at {}:{}", server_host, server_port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_manager.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .configure(handlers::auth::config)
                    .configure(handlers::users::config)
                    .configure(handlers::roles::config)
                    .configure(handlers::permissions::config)
                    .configure(handlers::departments::config)
                    .configure(handlers::employees::config)
                    .configure(handlers::projects::config)
                    .configure(handlers::upload::config)
            )
            .service(Files::new("/uploads", &upload_dir))
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
