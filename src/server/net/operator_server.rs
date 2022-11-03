use crate::server::controls::Controller;
use crate::share::{operatorpb, Error};
use operatorpb::operator_rpc_server::{OperatorRpc, OperatorRpcServer};
// use operatorpb::{listeners_request, listeners_response};
use operatorpb::{
    Empty, ImplantsResponse, ListImplants, ListenersRequest, ListenersResponse, NewTaskRequest,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

pub struct Listener {
    addr: SocketAddr,
}

impl Listener {
    pub fn new(addr: SocketAddr, ctl: Arc<Controller>) -> Result<Listener, Error> {
        let cert = std::fs::read("certs/server.pem")
            .map_err(|e| Error::FileReadErr("certs/server.pem".to_string(), e.to_string()))?;
        let key = std::fs::read("certs/server.key")
            .map_err(|e| Error::FileReadErr("certs/server.key".to_string(), e.to_string()))?;
        let identity = Identity::from_pem(cert, key);

        let service = OperatorService::new(ctl);
        let tls_config = ServerTlsConfig::new().identity(identity);
        let svc = OperatorRpcServer::new(service);

        let server = Server::builder()
            .tls_config(tls_config)
            .map_err(|_| Error::ListenerStartErr(addr.ip().to_string(), addr.port()))?
            .add_service(svc)
            .serve(addr);

        tokio::spawn(async move { server.await });

        Ok(Listener { addr })
    }
}

struct OperatorService {
    ctl: Arc<Controller>,
}

impl OperatorService {
    fn new(ctl: Arc<Controller>) -> OperatorService {
        OperatorService { ctl }
    }
}

#[tonic::async_trait]
impl OperatorRpc for OperatorService {
    async fn listeners(
        &self,
        request: Request<ListenersRequest>,
    ) -> Result<Response<ListenersResponse>, Status> {
        let response = self
            .ctl
            .listenerctl(request.into_inner())
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;

        Ok(Response::new(response))
    }

    async fn new_task(&self, request: Request<NewTaskRequest>) -> Result<Response<Empty>, Status> {
        let response = self
            .ctl
            .taskctl(request.into_inner())
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;

        Ok(Response::new(response))
    }

    async fn implants(
        &self,
        _: Request<ListImplants>,
    ) -> Result<Response<ImplantsResponse>, Status> {
        let response = self
            .ctl
            .implantctl()
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;
        Ok(Response::new(response))
    }
}
