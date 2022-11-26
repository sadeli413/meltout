use crate::share::operatorpb::{
    listeners_request, Empty, ListImplants, Listener, ListenersRequest, NewListener, NewTaskRequest,
};
use clap::{Parser, Subcommand};

// Compile an implant
#[derive(Parser, Debug)]
pub struct Generate {
    #[clap(long, value_parser)]
    pub lhost: String,

    #[clap(long, value_parser, default_value_t = 9001)]
    pub lport: u16,

    #[clap(long, value_parser, default_value_t = String::from("linux"))]
    pub os: String,
}

// Create a listener
#[derive(Parser)]
pub struct Listeners {
    #[clap(subcommand)]
    pub command: ListenersCommands,
}

#[derive(Subcommand)]
pub enum ListenersCommands {
    /// New listener at lhost:lport
    New {
        #[clap(long, value_parser)]
        lhost: String,

        #[clap(long, value_parser, default_value_t = 9001)]
        lport: u16,

        /// The server pem for HTTPS
        #[clap(long, value_parser)]
        server_pem: String,

        /// The server key for HTTPS
        #[clap(long, value_parser)]
        server_key: String,
    },
    List,
}

impl Listeners {
    pub fn to_protobuf(&self) -> ListenersRequest {
        let listeners_command = Some(match &self.command {
            ListenersCommands::New {
                lhost,
                lport,
                server_pem,
                server_key,
            } => {
                let listener = Listener {
                    id: "0".to_string(),
                    lhost: lhost.to_string(),
                    lport: *lport as u32,
                };

                listeners_request::ListenersCommand::NewListener(NewListener {
                    listener: Some(listener),
                    server_pem: server_pem.to_string(),
                    server_key: server_key.to_string(),
                })
            }

            ListenersCommands::List => listeners_request::ListenersCommand::ListListeners(Empty {}),
        });
        ListenersRequest { listeners_command }
    }
}

// Create a task for an implant
#[derive(Parser)]
pub struct Task {
    #[clap(long, value_parser)]
    pub cmd: String,

    #[clap(long, value_parser)]
    pub implant_id: String,
}

impl Task {
    pub fn to_protobuf(&self, operatorid: String) -> NewTaskRequest {
        NewTaskRequest {
            operatorid,
            cmd: self.cmd.clone(),
            implantid: self.implant_id.clone(),
        }
    }
}

#[derive(Parser)]
pub struct Implants {}

impl Implants {
    pub fn to_protobuf(&self) -> ListImplants {
        ListImplants {}
    }
}
