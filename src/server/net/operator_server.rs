mod operatorpb {
    tonic::include_proto!("operatorpb");
}

use crate::share::Error;
use operatorpb::{listeners_request, listeners_response};
use operatorpb::{Empty, ListenersRequest, ListenersResponse, NewListener, RepeatedNewListeners, TaskRequest, TaskResponse};
use operatorpb::operator_rpc_server::{OperatorRpc, OperatorRpcServer};
use prost::Message;
use std::net::SocketAddr;
use tonic::{Request, Response, Status};
use tonic::transport::{Identity, Server, ServerTlsConfig};

pub struct Listener {
    addr: SocketAddr
}

impl Listener {
    pub fn new(addr: SocketAddr) -> Listener {
        Listener { addr }
    }
}

impl Listener {
    pub fn start_listener(&self) -> Result<(), Error> {
        let cert = std::fs::read("certs/server.pem")
            .map_err(|e| Error::ListenerStartErr(e.to_string()))?;
        let key = std::fs::read("certs/server.key")
            .map_err(|e| Error::ListenerStartErr(e.to_string()))?;
        let identity =  Identity::from_pem(cert, key);

        let service = OperatorService::new();
        let tls_config = ServerTlsConfig::new().identity(identity);
        let svc = OperatorRpcServer::new(service);

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

struct OperatorService {}

impl OperatorService {
    fn new() -> OperatorService {
        OperatorService {  }
    }
}

#[tonic::async_trait]
impl OperatorRpc for OperatorService {
    async fn listeners(&self, request: Request<ListenersRequest>) -> Result<Response<ListenersResponse>, Status> {
        let request = request.into_inner();
        if let Some(cmd) = request.listeners_command {
            let response = match cmd {
                // Create a new listener
                listeners_request::ListenersCommand::NewListener(new_cmd) => {
                    let listeners_command = listeners_response::ListenersCommand::NewListener(Empty {});
                    let listeners_command = Some(listeners_command);
                    println!("{:?}", listeners_command);
                    Response::new(ListenersResponse {listeners_command})
                }

                // List listeners
                listeners_request::ListenersCommand::ListListeners(list_cmd) => {
                    let new_listeners = vec![NewListener {lhost: "foobar".to_string(), lport: 1337}];
                    let listeners_command = listeners_response::ListenersCommand::ListListeners(RepeatedNewListeners {new_listeners});
                    let listeners_command = Some(listeners_command);
                    Response::new(ListenersResponse {listeners_command})
                }
            };
            return Ok(response);
        } else {
            return Err(Status::unavailable("TODO: fancy error message"));
        }
    }

    async fn task(&self, _: Request<TaskRequest>) -> Result<Response<TaskResponse>, Status> {

        Ok(Response::new(TaskResponse { 
            stdout: vec![],
            stderr: vec![]
        }))
    }
}
