use clap::{Parser, Subcommand};

// Compile an implant
#[derive(Parser, Debug)]
pub struct Generate {
    #[clap(long, value_parser)]
    pub lhost: String,

    #[clap(long, value_parser, default_value_t = 9001)]
    pub lport: u16,

    #[clap(long, value_parser, default_value_t = String::from("linux"))]
    pub os: String
}

// Create a listener
#[derive(Parser)]
pub struct Listeners {
    #[clap(subcommand)]
    pub command: ListenersCommands
}

#[derive(Subcommand)]
pub enum ListenersCommands {
    // New listener at lhost:lport
    New {
        #[clap(long, value_parser)]
        lhost: String,

        #[clap(long, value_parser, default_value_t = 9001)]
        lport: u16
    },
    List
}

// Create a task for an implant
#[derive(Parser)]
pub struct Task {
    #[clap(long, value_parser)]
    pub cmd: String,

    #[clap(long, value_parser)]
    pub listener: String
}
