use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub role: Role,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    #[sea_orm(has_many)]
    pub api_keys: HasMany<super::api_key::Entity>
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "userRole", rename_all = "camelCase")]
pub enum Role {
    Admin,
    User
}