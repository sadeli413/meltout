use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220929_000002_create_tasks_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(Task::Table)
            .col(ColumnDef::new(Task::Uuid)
                 .uuid()
                 .not_null()
                 .primary_key()
            )
            .col(ColumnDef::new(Task::Ttype)
                 .integer()
                 .not_null()
            )
            .col(ColumnDef::new(Task::Payload).string())
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Task::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Task {
    Table,
    Uuid,
    Ttype,
    Payload
}
