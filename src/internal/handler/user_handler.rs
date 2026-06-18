
// src/internal/handler/user_handler.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::internal::constant::dto::{CreateUserRequest, ApiResponse};
use crate::internal::module::user_service::UserService;
use std::sync::Arc;
use uuid::Uuid;

/// Concrete implementation wrapper holding references to our business modules
pub struct UserHandler {
    user_service: Arc<dyn UserService>,
}

impl UserHandler {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }

    /// POST /api/v1/users
    pub async fn create_user(
        handler: web::Data<Self>, 
        payload: web::Json<CreateUserRequest>
    ) -> Result<impl Responder, crate::internal::constant::errors::AppError> {
        // Logging context automatically captures the trace IDs inherited from middleware
        tracing::info!(username = %payload.username, "HTTP Request received: Create User");

        let user = handler.user_service
            .register_new_user(payload.username.clone(), payload.email.clone())
            .await?;

        Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            data: user,
        }))
    }

    /// GET /api/v1/users/{id}
    pub async fn get_user(
        handler: web::Data<Self>,
        path: web::Path<String>
    ) -> Result<impl Responder, crate::internal::constant::errors::AppError> {
        let raw_id = path.into_inner();
        
        let target_uuid = Uuid::parse_str(&raw_id)
            .map_err(|_| crate::internal::constant::errors::AppError::ValidationError("Invalid UUID format".to_string()))?;

        tracing::info!(%target_uuid, "HTTP Request received: Get User by ID");

        let user = handler.user_service.get_user_by_id(target_uuid).await?;

        Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: user,
        }))
    }
}
