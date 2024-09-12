pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_a_table;
mod m20220101_000001_create_b_table;
mod m20220101_000001_create_c_table;
mod m20220101_000001_create_d_table;
mod m20220101_000001_create_e_table;
mod m20220101_000001_create_f_table;
mod m20220101_000001_create_junction_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_a_table::Migration),
            Box::new(m20220101_000001_create_b_table::Migration),
            Box::new(m20220101_000001_create_c_table::Migration),
            Box::new(m20220101_000001_create_d_table::Migration),
            Box::new(m20220101_000001_create_e_table::Migration),
            Box::new(m20220101_000001_create_f_table::Migration),
            Box::new(m20220101_000001_create_junction_table::Migration),
        ]
    }
}
