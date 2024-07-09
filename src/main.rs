mod handlers;

use actix_multipart::form::tempfile::TempFileConfig;

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("creating temporary upload directory");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
