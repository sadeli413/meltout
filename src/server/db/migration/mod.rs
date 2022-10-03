mod m20220929_000001_create_implants_table;
mod m20220929_000002_create_tasks_table;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220929_000001_create_implants_table::Migration),
            Box::new(m20220929_000002_create_tasks_table::Migration)
        ]
    }
}
