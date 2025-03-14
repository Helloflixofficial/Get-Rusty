//incomplete next day
use sea_orm::*;

#[derive(Debug,Clone, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  id: i32,
  name: String
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::post::Entity")]
  Post
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    
}
