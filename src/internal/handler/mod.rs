pub mod user_handler;

use actix_web::web;
use std::sync::Arc;

/// Route configuration registration block (Replaces your Go glue module)
pub fn configure_routes(cfg: &mut web::ServiceConfig, handlers: &crate::initiator::Handlers) {
    // Wrap handler controllers into actix-managed Data pointer references
    let user_handler_data = web::Data::new(user_handler::UserHandler::new(
        handlers.user_handler.clone(),
    ));

    cfg.service(
        web::scope("/api/v1")
            .app_data(user_handler_data)
            .route(
                "/users",
                web::post().to(user_handler::UserHandler::create_user),
            )
            .route(
                "/users/{id}",
                web::get().to(user_handler::UserHandler::get_user),
            ),
    );
}
