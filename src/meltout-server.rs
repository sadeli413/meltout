mod server;
mod share;

use std::sync::Arc;
use tokio::sync::Mutex;

use server::Server;
use share::Console;

const DB_NAME: &str = "sqlite:./meltout.db?mode=rwc";

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(
        server::db::Db::new(DB_NAME)
            .await
            .unwrap(),
    ));
    let mut console = Console::new();
    let ctl = server::controls::Controller::new(db);
    let (mut server, notifications_rx) = Server::new(Arc::new(ctl)).await;
    server::add_commands(&mut console);
    console.cli_loop(&mut server, notifications_rx);
}
