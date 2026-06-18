
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, Selectable};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::Pool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::internal::storage::UserRepository;
use crate::internal::storage::generated::schema;

// Define an alias for our high-performance bb8 connection pool
pub type DbPool = Pool<AsyncPgConnection>;

/// Database representation of a User record
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Structural representation for inserting a new user
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUserModel {
    pub username: String,
    pub email: String,
}




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
            .map_err(|_| diesel::result::Error::RollbackTransaction)?;

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
            .map_err(|_| diesel::result::Error::RollbackTransaction)?;

        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<UserModel>(&mut conn)
            .await
    }
}
