mod net;
mod commands;
mod parsers;
use std::collections::HashMap;

pub use commands::add_commands;

pub struct Server {
    listeners: HashMap<String, net::implant_server::Listener>
}

impl Server {
    pub fn new() -> Server {
        Server { listeners: HashMap::new() }
    }
}

