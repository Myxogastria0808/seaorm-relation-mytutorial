use crate::m20220101_000001_create_c_table::SampleC;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SampleD::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SampleD::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SampleD::Value).string().not_null())
                    .col(ColumnDef::new(SampleD::SampleCId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SampleD::Table, SampleD::SampleCId)
                            .to(SampleC::Table, SampleC::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SampleD::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SampleD {
    Table,
    Id,
    SampleCId,
    Value,
}
