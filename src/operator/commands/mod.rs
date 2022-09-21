use clap::Parser;
use tokio::runtime::Handle;
use crate::share::{Commands, Console, parsers, Error};
use super::Operator;
use super::net::operatorpb::{ListenersRequest, listeners_request::ListenersCommand, NewListener};

impl Commands for Operator {
    // TODO: Implant generation
    fn generate(&self, parser: parsers::Generate) -> Result<(), Error> {
        println!("{:?}", parser);
        Ok(())
    }

    fn listeners(&mut self, parser: parsers::Listeners) -> Result<(), Error> {
        match &parser.command {
            parsers::ListenersCommands::New { lhost, lport } => {
                let request = tonic::Request::new(ListenersRequest {
                    listeners_command: Some(ListenersCommand::NewListener(NewListener {
                        lhost: lhost.to_string(),
                        lport: *lport as u32
                    }))
                });
                
                // Run the async method
                let handle = Handle::current();
                let _guard = handle.enter();
                let response = futures::executor::block_on(self.rpc.listeners(request))
                    .map_err(|e| Error::ServerConnectErr(e.to_string()))?
                    .into_inner();
            }
            parsers::ListenersCommands::List => {

            }
        }
        Ok(())
    }

    fn task(&mut self, parser: parsers::Task) -> Result<(), Error> {
        Ok(())
    }
}

pub fn add_commands(console: &mut Console<Operator>) {
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
