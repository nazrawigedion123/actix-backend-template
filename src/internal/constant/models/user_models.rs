use crate::internal::storage::generated::schema;
use chrono::{DateTime, Utc};

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::{ Insertable, Queryable, Selectable};
/// Database representation of a User record (Query Output)
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

/// Structural representation for inserting a new user (Query Input)
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUserModel {
    pub username: String,
    pub email: String,
}