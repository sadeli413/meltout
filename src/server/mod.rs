pub mod db;
mod net;
mod commands;
pub mod controls;
mod parsers;
use std::collections::HashMap;
use std::sync::Arc;

pub use commands::add_commands;

pub struct Server {
    listeners: HashMap<String, net::implant_server::Listener>,
    db: Arc<db::Db>
}

impl Server {
    pub fn new(db: Arc<db::Db>) -> Server {
        Server { 
            listeners: HashMap::new(),
            db
        }
    }
}
