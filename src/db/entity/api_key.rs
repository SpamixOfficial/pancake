use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "api_key")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub key: String,
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
    // TODO: Maybe support for granular api keys in the future?
    pub permissions_like: super::user::Role // eg. User role would translate to the same permissions as a User
}

impl ActiveModelBehavior for ActiveModel {}