//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use crate::junction;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sample_e")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::junction::Entity")]
    Junction,
}

impl Related<super::sample_f::Entity> for Entity {
    fn to() -> RelationDef {
        junction::Relation::SampleF.def()
    }
    fn via() -> Option<RelationDef> {
        Some(junction::Relation::SampleE.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
