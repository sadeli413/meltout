use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "implant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: uuid::Uuid,
    pub task: Uuid
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}
