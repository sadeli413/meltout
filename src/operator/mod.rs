mod commands;
mod net;

use crate::share::Error;
use crate::share::operatorpb::operator_rpc_client::OperatorRpcClient;
use tonic::transport::Channel;

pub use commands::add_commands;

pub struct Operator {
    rpc: OperatorRpcClient<Channel>,
}

impl Operator {
    pub async fn new(server_url: String) -> Result<Operator, Error> {
        let rpc = net::new_rpcclient(server_url).await?;
        Ok(Operator { rpc })
    }
}
