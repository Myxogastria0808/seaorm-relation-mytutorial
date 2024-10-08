//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sample_b")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub value: String,
    #[sea_orm(unique)]
    pub sample_a_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sample_a::Entity",
        from = "Column::SampleAId",
        to = "super::sample_a::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SampleA,
}

impl Related<super::sample_a::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SampleA.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
