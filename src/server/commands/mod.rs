use std::sync::Arc;

use clap::Parser;
use super::Server;
use super::net::{implant_server, operator_server};
use crate::share::{Console, Commander, Error, parsers};

// General purpose commands for the server
impl Commander for Server {
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
                let listener = implant_server::Listener::new(addr, Arc::clone(&self.db));
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
        // let listener = self.listeners.get_mut(&parser.listener)
            // .ok_or_else(|| Error::ListenerNotExist(parser.listener.clone()))?;

        // let mut task = listener.db.lock()
        //     .map_err(|_| {
        //         let name = format!("listener {}", parser.listener.clone());
        //         Error::LockMutex(name)
        //     })?;
        futures::executor::block_on(self.db.add_task(super::db::entities::tasks::Model {
            uuid: uuid::Uuid::new_v4(),
            ttype: crate::share::implantpb::TaskType::ExecTask as i32,
            payload: Some(parser.cmd.clone())
        }))
        .map_err(|e| Error::DatabaseErr(e))?;

        println!("Added `{}` to tasks", parser.cmd);
        // task.insert(0, parser.cmd);
        Ok(())
    }
}

// Special commands for the server
impl Server {
    fn operators(&self, parser: super::parsers::Operators) -> Result<(), Error> {
        match &parser.command {
            super::parsers::OperatorsCommands::Enable { lhost, lport } => {
                let ip = lhost.parse()
                    .map_err(|_| Error::InvalidIP(lhost.to_string()))?;
                let addr = std::net::SocketAddr::new(ip, *lport);
                let listener = operator_server::Listener::new(addr);
                match listener.start_listener() {
                    Ok(_) => println!("Started listening for operators on {}:{}", lhost, lport),
                    Err(err) => eprintln!("Error, could listen for operators on {}:{}\n{:?}", lhost, lport, err)
                }
            }
        }
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
    console.commands.insert(
        "operators".to_string(), 
        |meltout, line| {
            let parser = super::parsers::Operators::try_parse_from(line)
                .map_err(|e| Error::CommandParsingErr(e))?;
            meltout.operators(parser)?;
            Ok(())
        }
    );
}
