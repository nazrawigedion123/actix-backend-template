
// src/internal/module/user_service.rs
use crate::internal::constant::errors::AppError;
use crate::internal::storage::user_storage::{NewUserModel, UserModel, UserRepository};
use std::sync::Arc;
use uuid::Uuid;

/// Business operations domain specification interface
#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> Result<UserModel, AppError>;
    async fn register_new_user(&self, username: String, email: String) -> Result<UserModel, AppError>;
}

/// Concrete implementation holding our repository trait object wrapper
pub struct DefaultUserService {
    // We use dyn UserRepository to facilitate effortless service level mocking
    user_repo: Arc<dyn UserRepository>,
}

impl DefaultUserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait::async_trait]
impl UserService for DefaultUserService {
    async fn get_user_by_id(&self, target_id: Uuid) -> Result<UserModel, AppError> {
        tracing::debug!(%target_id, "Executing domain rule: get_user_by_id verification checks");

        match self.user_repo.find_by_id(target_id).await? {
            Some(user) => Ok(user),
            None => Err(AppError::NotFound),
        }
    }

    async fn register_new_user(&self, username: String, email: String) -> Result<UserModel, AppError> {
        tracing::info!(%username, %email, "Executing domain rule: register_new_user constraints validation");

        // Example business validation check
        if username.trim().is_empty() || !email.contains('@') {
            return Err(AppError::ValidationError("Malformed parameter inputs supplied".to_string()));
        }

        let new_user = NewUserModel { username, email };
        let created_user = self.user_repo.create_user(new_user).await?;

        Ok(created_user)
    }
}
