use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use tracing::info;

mod config;
mod db;
mod mqtt;
mod auth;
mod errors;
mod state;
mod handlers;
mod routes;

use config::AppConfig;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Home Visual Backend Server...");

    let config = AppConfig::from_env().expect("Failed to load configuration");

    let db = db::init_db(&config)
        .await
        .expect("Failed to initialize MongoDB connection");

    let mqtt = mqtt::init_mqtt(&config)
        .await
        .expect("Failed to initialize MQTT connection");

    let state = AppState {
        db,
        mqtt,
        config: config.clone(),
    };

    let app_data = web::Data::new(state);

    let server_url = format!("{}:{}", config.server_host, config.server_port);
    info!("Starting HTTP server at http://{}", server_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .configure(routes::init)
    })
    .bind(server_url)?
    .run()
    .await
}
