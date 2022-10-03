mod server;
mod share;

use std::sync::Arc;

use server::Server;
use share::Console;


#[tokio::main]
async fn main() {
    let db = server::db::Db::new("sqlite:./meltout.db?mode=rwc").await.unwrap();
    let mut server = Server::new(Arc::new(db));
    let mut console = Console::new();
    // server::controls::test().await;
    server::add_commands(&mut console);
    console.cli_loop(&mut server);
}
