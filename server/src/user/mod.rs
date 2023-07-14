use chrono::{DateTime, Utc};

use self::pb::User;
use self::pb::user::{OptionalName, OptionalUpdatedAt, OptionalDeletedAt};

pub mod pb {
    tonic::include_proto!("user");
}

mod repository;
pub mod service;

#[derive(sqlx::FromRow)]
struct UserRecord {
    id: i32,
    email: String,
    username: String,
    name: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl UserRecord {
    fn to_proto(self) -> User {
        User {
            id: self.id,
            email: self.email,
            username: self.username,
            created_at: self.created_at.to_string(),

            optional_name: match self.name {
                Some(name) => Some(OptionalName::Name(name)),
                None => None,
            },
            optional_updated_at: match self.updated_at {
                Some(timestamp) => Some(OptionalUpdatedAt::UpdatedAt(timestamp.to_string())),
                None => None,
            },
            optional_deleted_at: match self.deleted_at {
                Some(timestamp) => Some(OptionalDeletedAt::DeletedAt(timestamp.to_string())),
                None => None,
            }
        }
    }
}
