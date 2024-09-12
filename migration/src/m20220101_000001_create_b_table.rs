use crate::m20220101_000001_create_a_table::SampleA;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SampleB::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SampleB::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SampleB::Value).string().not_null())
                    .col(
                        ColumnDef::new(SampleB::SampleAId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SampleB::Table, SampleB::SampleAId)
                            .to(SampleA::Table, SampleA::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SampleB::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SampleB {
    Table,
    Id,
    SampleAId,
    Value,
}
