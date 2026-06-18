// pub mod routes;
use crate::internal::handler::user_handler::UserHandler;
use actix_web::{web, };
use actix_web_lab::middleware::{from_fn};
use crate::initiator::Handlers;
use crate::internal::handler::middleware::{append_trace_header_middleware,dummy_auth_middleware};



/// Central configuration controller entry point invoked by src/main.rs
pub fn configure_routes(cfg: &mut web::ServiceConfig, handlers: &Handlers) {
    let user_handler = web::Data::new(
        UserHandler::new(handlers.user_handler.clone())
    );

    cfg.service(
        web::scope("")
            .app_data(user_handler.clone())
            .route(
                "/api/v1/users",
                web::post()
                    .to(UserHandler::create_user)
                    .wrap(from_fn(append_trace_header_middleware)),
            )
            .route(
                "/api/v1/users/{id}",
                web::get()
                    .to(UserHandler::get_user)
                    .wrap(from_fn(append_trace_header_middleware))
                    .wrap(from_fn(dummy_auth_middleware)),
            ),
    );
}
