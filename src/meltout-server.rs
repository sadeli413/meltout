mod server;
mod share;

use std::sync::Arc;
use tokio::sync::Mutex;

use server::Server;
use share::Console;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(
        server::db::Db::new("sqlite:./meltout.db?mode=rwc")
            .await
            .unwrap(),
    ));
    let ctl = Arc::new(server::controls::Controller::new(db));
    let mut server = Server::new(ctl);
    let mut console = Console::new();
    server::add_commands(&mut console);
    console.cli_loop(&mut server);
}
