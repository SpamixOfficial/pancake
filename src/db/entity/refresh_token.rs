use chrono::Utc;
use migration::async_trait;
use sea_orm::{ActiveValue::{NotSet, Set}, entity::prelude::*};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_token")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub token: String,
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
    pub expires: DateTimeUtc,
    // Session info
    pub device_info_string: Option<String>,
    pub created_at: DateTimeUtc,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.created_at = Set(Utc::now().into());
        } else {
            self.created_at = NotSet
        }

        Ok(self)
    }
}
