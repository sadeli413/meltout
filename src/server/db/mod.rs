mod migration;
pub mod entities;

use migration::Migrator;
use sea_orm::{DbErr, Database, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use sea_orm_migration::MigratorTrait;

pub struct Db {
    conn: DatabaseConnection
}

impl Db {
    pub async fn new(url: &str) -> Result<Db, DbErr> {
        let conn = Database::connect(url).await?;
        // let _manager = SchemaManager::new(&conn);
        Migrator::refresh(&conn).await?;
        Ok(Db {conn})
    }

    pub async fn add_task(&self, task: entities::tasks::Model) -> Result<(), DbErr>{
        let task = entities::tasks::ActiveModel {
            uuid: Set(task.uuid),
            ttype: Set(task.ttype),
            payload: Set(task.payload),
        };
        entities::tasks::Entity::insert(task).exec(&self.conn).await?;
        Ok(())
    }

    pub async fn get_task(&self) -> Result<Option<entities::tasks::Model>, DbErr>{
        let task = entities::tasks::Entity::find()
            .all(&self.conn)
            .await?;
        let task = task.last().cloned();
        Ok(task)
    }
}

