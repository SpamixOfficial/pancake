//! Models that do not belong to any specific category

use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    user_id: i32,
    expires_at: DateTimeUtc
}