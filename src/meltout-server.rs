mod server;
mod share;

use server::Server;
use share::Console;


#[tokio::main]
async fn main() {
    let mut server = Server::new();
    let mut console = Console::new();
    server::add_commands(&mut console);
    console.cli_loop(&mut server);
}
