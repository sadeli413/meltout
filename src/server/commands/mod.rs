use std::sync::Arc;

use super::net::operator_server;
use super::Server;
use crate::share::parsers as share_parsers;
use crate::share::{Commander, Console, Error, operatorpb};
use clap::Parser;
use paste::paste;
use futures::executor::block_on;
use prettytable::row;
use super::parsers as server_parsers;

// General purpose commands for the server
impl Commander for Server {
    // TODO: Generate implants
    fn generate(&self, parser: share_parsers::Generate) -> Result<(), Error> {
        println!("{:?}", parser);
        Ok(())
    }

    fn listeners(&mut self, parser: share_parsers::Listeners) -> Result<(), Error> {
        let request = parser.to_protobuf();
        let response = block_on(self.ctl.listenerctl(request))?;
        if let Some(cmd) = response.listeners_command {
            match cmd {
                operatorpb::listeners_response::ListenersCommand::NewListener(_) => {
                    println!("Listener successfully added.")
                }
                operatorpb::listeners_response::ListenersCommand::ListListeners(listeners) => {
                    let mut table = prettytable::Table::new();
                    table.add_row(row!["ID", "LHOST", "LPORT"]);

                    for l in listeners.new_listeners {
                        table.add_row(row![l.id, l.lhost, l.lport]);
                    }
                    table.printstd();
                }
            }
        }
        Ok(())
    }

    fn task(&mut self, parser: share_parsers::Task) -> Result<(), Error> {
        let request = parser.to_protobuf();
        block_on(self.ctl.taskctl(request))?;

        println!("Added `{}` to tasks", parser.cmd);
        Ok(())
    }

    fn implants(&mut self, _: share_parsers::Implants) -> Result<(), Error> {
        let response = block_on(self.ctl.implantctl())?;
        for ele in response.implants {
            println!("{}", ele)
        }
        Ok(())
    }
}

// Special commands for the server
impl Server {
    fn operators(&self, parser: server_parsers::Operators) -> Result<(), Error> {
        match &parser.command {
            server_parsers::OperatorsCommands::Enable { lhost, lport } => {
                let ip = lhost
                    .parse()
                    .map_err(|_| Error::InvalidIP(lhost.to_string()))?;
                let addr = std::net::SocketAddr::new(ip, *lport);
                match operator_server::Listener::new(addr, Arc::clone(&self.ctl)) {
                    Ok(_) => println!("Started listening for operators on {}:{}", lhost, lport),
                    Err(err) => eprintln!(
                        "Error, could listen for operators on {}:{}\n{:?}",
                        lhost, lport, err
                    ),
                }
            }
        }
        Ok(())
    }
}

// Add commands to the console
pub fn add_commands(console: &mut Console<Server>) {
    crate::add_command!(console, share_parsers, generate);
    crate::add_command!(console, share_parsers, listeners);
    crate::add_command!(console, share_parsers, task);
    crate::add_command!(console, share_parsers, implants);
    crate::add_command!(console, server_parsers, operators);
}

