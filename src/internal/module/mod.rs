pub mod user_service;
use crate::internal::constant::errors::AppError;
use crate::internal::constant::models::user_models::{UserModel};

use uuid::Uuid;
#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> Result<UserModel, AppError>;
    async fn register_new_user(&self, username: String, email: String) -> Result<UserModel, AppError>;
}
