pub mod internal;
use config::AppConfig;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    internal::platform::logger::init_logger();
    info!("Logger layyer attached. Initializing service depenencies");
    let cfg = AppConfig::local_from_env();
    info!(
        port = cfg.server_port,
        db_connected = !cfg.database_url.is_empty(),
        "Configuration set up loaded successfully via envy enviroment"
    );
    Ok(())
}
