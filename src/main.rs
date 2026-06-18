/*mod config;
mod internal;

mod initiator;


use config::AppConfig;
use tracing::info;
use initiator::AppInitiator;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // initalize structural logging
    internal::platform::logger::init_logger();
    // load setting via envy 
    info!("Logger layyer attached. Initializing service depenencies");
    let cfg = AppConfig::load_from_env();
    info!(
        port = cfg.server_port,
        db_connected = !cfg.database_url.is_empty(),
        "Configuration set up loaded successfully via envy enviroment"
    );
    let initiator =AppInitiator::init(cfg).await;
    info!("System initialization phase complete. Ready to boot web servers");

    Ok(())
}*/



// src/main.rs
pub mod config;
pub mod initiator;
pub mod internal;

use actix_web::{App, HttpServer};
use config::AppConfig;
use initiator::AppInitiator;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Boot up global structured telemetry logging 
    internal::platform::logger::init_logger();

    // 2. Load system configurations safely via Envy environment layers
    let cfg = AppConfig::load_from_env();

    // 3. Complete structural dependency tree orchestration pipelines (Storage -> Modules -> Handlers)
    let initiator = AppInitiator::init(cfg).await;
    
    // Cache port configuration for runtime closures
    let bind_port = cfg.server_port;

    tracing::info!("Starting actix production server cluster on port {}...", bind_port);

    HttpServer::new(move || {
        // Clone a copy of our fully wired initiator tree into every parallel worker thread context
        let app_initiator = initiator.clone();

        App::new()
            // Attach our advanced asynchronous TracingLogger middleware (automatically traces request context)
            .wrap(TracingLogger::default())
            // Pass configuration route mappings down to mount all application handler endpoints
            .configure(|web_cfg| internal::handler::configure_routes(web_cfg, &app_initiator.handlers))
    })
    .bind(("127.0.0.1", bind_port))?
    .run()
    .await
}
