mod config;
mod db;
mod errors;
mod models;
mod routes;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cfg = config::Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await;

    log::info!("Starting server on {}:{}", cfg.host, cfg.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(1024 * 1024)) // 1MB JSON limit
            .wrap(middleware::Logger::default())
            .configure(routes::submission::configure)
    })
    .bind((cfg.host.as_str(), cfg.port))?
    .run()
    .await
}
