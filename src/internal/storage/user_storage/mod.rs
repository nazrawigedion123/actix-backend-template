
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use crate::internal::storage::UserRepository;

use uuid::Uuid;
use crate::internal::constant::models::user_models::{NewUserModel, UserModel};


// Define an alias for our high-performance bb8 connection pool
pub type DbPool = Pool<AsyncPgConnection>;



/// Concrete implementation wrapper holding the thread-safe connection pool
pub struct DieselUserRepository {
    pub pool: DbPool,
}

impl DieselUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for DieselUserRepository {
    async fn find_by_id(
        &self,
        target_id: Uuid,
    ) -> Result<Option<UserModel>, diesel::result::Error> {
        use crate::internal::storage::generated::schema::users::dsl::*;
        use diesel_async::RunQueryDsl;

        // Trace logging automatically inherits request IDs via the async context
        tracing::debug!(%target_id, "Executing find_by_id query in database context");

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| diesel::result::Error::RollbackTransaction)?; // Simplified error translation

        let result = users
            .filter(id.eq(target_id))
            .first::<UserModel>(&mut conn)
            .await;

        match result {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn create_user(
        &self,
        new_user: NewUserModel,
    ) -> Result<UserModel, diesel::result::Error> {
        use crate::internal::storage::generated::schema::users::dsl::*;
        use diesel_async::RunQueryDsl;

        tracing::info!(username = %new_user.username, "Persisting new record into user storage");

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| diesel::result::Error::RollbackTransaction)?;

        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<UserModel>(&mut conn)
            .await
    }
}
