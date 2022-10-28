mod commands;
pub mod controls;
pub mod db;
mod net;
mod parsers;

use std::sync::Arc;

pub use commands::add_commands;

pub struct Server {
    ctl: Arc<controls::Controller>,
}

impl Server {
    pub fn new(ctl: Arc<controls::Controller>) -> Server {
        Server { ctl }
    }
}
