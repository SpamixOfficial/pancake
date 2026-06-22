//! Models that do not belong to any specific category

use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    user_id: i64,
    expires_at: OffsetDateTime
}

impl UserSession {
    pub fn new(user_id: i64, expiration: Option<Duration>) -> Self {
        let expires_at = OffsetDateTime::now_utc() + expiration.unwrap_or(Duration::minutes(30));
        Self { user_id, expires_at }
    }
}