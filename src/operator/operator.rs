use crate::share::operatorpb::operator_rpc_client::OperatorRpcClient;
use crate::share::operatorpb::{Notification, Registration};
use crate::share::Error;
use tokio::sync::mpsc::{channel, Receiver};
use tonic::transport::Channel;
use tonic::{Status, Streaming};

pub use super::commands::add_commands;

pub struct Operator {
    pub rpc: OperatorRpcClient<Channel>,
    pub id: String,
}

impl Operator {
    pub async fn new(
        server_url: String,
        rootca: String,
        domain_name: String,
    ) -> Result<(Operator, Receiver<Result<Notification, Status>>), Error> {
        let mut rpc = super::net::new_rpcclient(server_url, rootca, domain_name).await?;
        let request = Registration {};
        let confirmation = rpc
            .register(request)
            .await
            .map_err(|e| Error::RpcError(e))?
            .into_inner();

        let id = confirmation.operator_id.clone();
        let notifications = rpc
            .notifications(confirmation)
            .await
            .map_err(|e| Error::RpcError(e))?
            .into_inner();

        Ok((Operator { rpc, id }, stream2channel(notifications).await))
    }
}

// Convert the notification stream to a channel
async fn stream2channel(
    mut notifications: Streaming<Notification>,
) -> Receiver<Result<Notification, Status>> {
    let (tx, rx) = channel(1);
    tokio::spawn(async move {
        while let Some(notification) = notifications.message().await.transpose() {
            let _ = tx.send(notification).await;
        }
    });
    rx
}
