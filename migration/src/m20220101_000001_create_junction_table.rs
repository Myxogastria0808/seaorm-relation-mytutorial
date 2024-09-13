use crate::m20220101_000001_create_e_table::SampleE;
use crate::m20220101_000001_create_f_table::SampleF;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Junction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Junction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Junction::SampleEId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Junction::Table, Junction::SampleEId)
                            .to(SampleE::Table, SampleE::Id),
                    )
                    .col(ColumnDef::new(Junction::SampleFId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Junction::Table, Junction::SampleFId)
                            .to(SampleF::Table, SampleF::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Junction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Junction {
    Table,
    Id,
    SampleEId,
    SampleFId,
}
