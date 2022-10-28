use clap::Parser;
// use tokio::runtime::Handle;
// use crate::share::operatorpb::{listeners_request::ListenersCommand, ListenersRequest, NewListener, Empty};
use crate::share::operatorpb::listeners_response::ListenersCommand::{NewListener, ListListeners};
use crate::share::{parsers, Commander, Console, Error};
use futures::executor::block_on;
use paste::paste;
use prettytable::row;
use super::Operator;

impl Commander for Operator {
    // TODO: Implant generation
    fn generate(&self, parser: parsers::Generate) -> Result<(), Error> {
        println!("{:?}", parser);
        Ok(())
    }

    // Listen for implants
    fn listeners(&mut self, parser: parsers::Listeners) -> Result<(), Error> {
        let request = parser.to_protobuf();
        let response = block_on(self.rpc.listeners(request))
            .map_err(|e| Error::ServerConnectErr(e.to_string()))?
            .into_inner();
        
        // Print out the response
        if let Some(cmd) = response.listeners_command {
            match cmd {
                NewListener(_) => {
                    println!("Listener successfully added.")
                }
                ListListeners(listeners) => {
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

    // Create a task
    fn task(&mut self, parser: parsers::Task) -> Result<(), Error> {
        let request = parser.to_protobuf();
        block_on(self.rpc.new_task(request))
            .map_err(|e| Error::ServerConnectErr(e.to_string()))?;

        println!("Added `{}` to tasks", parser.cmd);
        Ok(())
    }

    // List out implants
    fn implants(&mut self, parser: parsers::Implants) -> Result<(), Error> {
        let request = parser.to_protobuf();
        let response = block_on(self.rpc.implants(request))
            .map_err(|e| Error::ServerConnectErr(e.to_string()))?
            .into_inner();
        for ele in response.implants {
            println!("{}", ele)
        }
        Ok(())
    }
}

pub fn add_commands(console: &mut Console<Operator>) {
    crate::add_command!(console, parsers, generate);
    crate::add_command!(console, parsers, listeners);
    crate::add_command!(console, parsers, task);
    crate::add_command!(console, parsers, implants);
}
