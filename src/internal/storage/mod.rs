
 pub mod user_storage;
pub mod generated;


use uuid::Uuid;
use crate::internal::constant::models::user_models::{NewUserModel, UserModel};




#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, diesel::result::Error>;
    async fn create_user(&self, new_user: NewUserModel)
    -> Result<UserModel, diesel::result::Error>;
}
