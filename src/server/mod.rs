mod net;
mod commands;

use std::collections::HashMap;

pub struct Server {
    listeners: HashMap<String, net::implant_server::Listener>
}

pub use commands::add_commands;
