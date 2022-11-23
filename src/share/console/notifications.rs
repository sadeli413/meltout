use std::io::Write;
use tonic::Status;
use tokio::sync::mpsc::Receiver;

use crate::share::{Error, operatorpb::Notification};

/// Constant stream of notifications
pub async fn notification_loop(mut notifications_rx: Receiver<Result<Notification, Status>>) {
    while let Some(notification) = notifications_rx.recv().await {
        if let Err(e) = display_notification(notification) {
            eprintln!("{}", e);
        }
    }
}

// Display a notification (usually just the result of a task)
fn display_notification(notification: Result<Notification, Status>) -> Result<(), Error> {
    let n = notification.map_err(|e| Error::RpcError(e))?;
    std::io::stdout()
        .write_all(&n.stdout)
        .map_err(|e| Error::IOErr(e))?;
    std::io::stderr()
        .write_all(&n.stderr)
        .map_err(|e| Error::IOErr(e))?;
    Ok(())
}
