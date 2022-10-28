use crate::share::operatorpb;

// use std::net::SocketAddr;

// use clap::Parser;
// use crate::share::parsers;
use operatorpb::operator_rpc_client::OperatorRpcClient;
// use operatorpb::listeners_request;
// use operatorpb::{ListenersRequest, ListenersResponse};
use crate::share::Error;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Uri};

pub async fn new_rpcclient(server_url: String) -> Result<OperatorRpcClient<Channel>, Error> {
    let pem = tokio::fs::read("certs/ca.pem")
        .await
        .map_err(|e| Error::ServerConnectErr(e.to_string()))?;

    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("ZHJvcGxldHNlcnZlciAK.YzIK");

    // let channel = Channel::from_static(url)
    let uri = server_url
        .parse::<Uri>()
        .map_err(|e| Error::ServerConnectErr(e.to_string()))?;

    let channel = Channel::builder(uri)
        .tls_config(tls)
        .map_err(|e| Error::ServerConnectErr(e.to_string()))?
        .connect()
        .await
        .map_err(|e| Error::ServerConnectErr(e.to_string()))?;

    let client = OperatorRpcClient::new(channel);
    Ok(client)
}
