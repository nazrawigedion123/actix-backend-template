// src/main.rs
pub mod config;
pub mod initiator;
pub mod internal;


use initiator::AppInitiator;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    AppInitiator::initiate().await
    
    
}
