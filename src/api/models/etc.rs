//! Models that do not belong to any specific category

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    user_id: i64,
    expires_at: DateTime<Utc>
}

impl UserSession {
    pub fn new(user_id: i64, expiration: Option<Duration>) -> Self {
        let expires_at = Utc::now() + expiration.unwrap_or(Duration::minutes(30));
        Self { user_id, expires_at }
    }
}