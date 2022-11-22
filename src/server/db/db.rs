use super::entities::{implants, tasks};
use super::migration::Migrator;
use crate::server::net::implant_server::Listener;
use crate::share::{implantpb, operatorpb, Error};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait, Set};
use sea_orm_migration::MigratorTrait;
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};

// A database to store application-wide data
// Contains a sqlite3 database along with several in-memory application data
pub struct Db {
    conn: DatabaseConnection,
    // A List of tasks
    tasks: Vec<tasks::Task>,
    pub listeners: Vec<Listener>,
    // Hashmap of operators and their notification channels
    operators_tx: HashMap<String, Sender<Result<operatorpb::Notification, tonic::Status>>>,
    operators: Vec<String>,
}

impl Db {
    pub async fn new(url: &str) -> Result<Db, DbErr> {
        let conn = Database::connect(url).await?;
        // let _manager = SchemaManager::new(&conn);
        Migrator::refresh(&conn).await?;
        Ok(Db {
            conn,
            tasks: Vec::new(),
            listeners: Vec::new(),
            operators_tx: HashMap::new(),
            operators: Vec::new(),
        })
    }

    pub fn add_task(&mut self, task: tasks::Task) {
        self.tasks.push(task);
    }

    pub fn pop_task(&mut self, task_id: String) -> Option<tasks::Task> {
        self.tasks.iter()
            .position(|t| t.task.task_id == task_id)
            .map(|i| self.tasks.swap_remove(i))
    }

    // Search for the implant_id and pop it's task
    pub fn search_task_by_implant(&self, implant_id: String) -> Option<implantpb::TaskResponse> {
        self.tasks
            .iter()
            .find(|t| t.implant_id == implant_id)
            .map(|t| t.task.clone())
    }

    pub async fn register_implant(
        &self,
        _registration: implantpb::Registration,
    ) -> Result<uuid::Uuid, Error> {
        let uuid = uuid::Uuid::new_v4();
        let registration = implants::ActiveModel { uuid: Set(uuid) };

        registration
            .insert(&self.conn)
            .await
            .map_err(|e| Error::DatabaseErr(e))?;

        Ok(uuid)
    }

    pub async fn list_implants(&self) -> Result<Vec<String>, Error> {
        Ok(implants::Entity::find()
            .all(&self.conn)
            .await
            .map_err(|e| Error::DatabaseErr(e))?
            .iter()
            .map(|m| m.uuid.to_string())
            .collect())
    }

    pub fn register_server(&mut self) -> String {
        let zero = String::from("0");
        self.operators.push(zero.clone());
        zero
    }

    pub fn register_operator(&mut self) -> String {
        let uuid = uuid::Uuid::new_v4().to_string();
        self.operators.push(uuid.clone());
        uuid
    }

    pub fn init_notifications(
        &mut self,
        operator_id: String,
    ) -> Receiver<Result<operatorpb::Notification, tonic::Status>> {
        let (tx, rx) = channel(1);
        self.operators_tx.insert(operator_id, tx);
        rx
    }

    pub async fn push_notification(
        &self,
        operator_id: String,
        task_result: implantpb::TaskResult,
    ) -> Result<(), Error> {
        // Convert TaskResult to Notification
        let notification = operatorpb::Notification {
            stdout: task_result.stdout,
            stderr: task_result.stderr,
        };
        Ok(self
            .operators_tx
            .get(&operator_id)
            .ok_or_else(|| Error::OperatorNotFound(operator_id))?
            .send(Ok(notification))
            .await
            .map_err(|e| Error::ChannelErr(e.to_string()))?)
    }
}
