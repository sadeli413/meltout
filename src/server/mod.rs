mod commands;
pub mod controls;
pub mod db;
mod net;
mod parsers;

use crate::share::operatorpb::Notification;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tonic::Status;

pub use commands::add_commands;

pub struct Server {
    ctl: Arc<controls::Controller>,
}

impl Server {
    pub async fn new(
        ctl: Arc<controls::Controller>,
    ) -> (Server, Receiver<Result<Notification, Status>>) {
        let confirmation = ctl.register_server().await;
        let rx = ctl.notifications(confirmation).await;
        (Server { ctl }, rx)
    }
}
