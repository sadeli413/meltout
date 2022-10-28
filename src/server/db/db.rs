use std::collections::HashMap;
use super::entities::implants;
use super::migration::Migrator;
use crate::server::net::implant_server::Listener;
use crate::share::{implantpb, Error};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait, Set};
use sea_orm_migration::MigratorTrait;

pub struct Db {
    conn: DatabaseConnection,
    tasks: HashMap<String, implantpb::TaskResponse>,
    pub listeners: Vec<Listener>,
}

impl Db {
    pub async fn new(url: &str) -> Result<Db, DbErr> {
        let conn = Database::connect(url).await?;
        // let _manager = SchemaManager::new(&conn);
        Migrator::refresh(&conn).await?;
        let tasks = HashMap::new();
        Ok(Db {
            conn,
            tasks,
            listeners: vec![],
        })
    }

    // TODO: Don't override existing tasks
    pub fn add_task(&mut self, implant_id: String, task: implantpb::TaskResponse) {
        self.tasks.insert(implant_id, task);
    }

    pub fn pop_task(&mut self, implant_id: String) -> Option<implantpb::TaskResponse> {
        self.tasks.remove(&implant_id)
    }

    pub async fn register_implant(
        &self,
        _registration: implantpb::Registration,
    ) -> Result<(), Error> {
        let registration = implants::ActiveModel {
            uuid: Set(uuid::Uuid::new_v4())
        };

        registration
            .insert(&self.conn)
            .await
            .map_err(|e| Error::DatabaseErr(e))?;

        Ok(())
    }

    pub async fn list_implants(&self) -> Result<Vec<String>, Error> {
        Ok(implants::Entity::find()
            .all(&self.conn)
            .await
            .map_err(|e| Error::DatabaseErr(e))?
            .into_iter()
            .map(|m| m.uuid.to_string())
            .collect())
    }
}
