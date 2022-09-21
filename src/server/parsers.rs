use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Operators {
    #[clap(subcommand)]
    pub command: OperatorsCommands
}

#[derive(Subcommand)]
pub enum OperatorsCommands {
    // Listen for operators on lhost:lport
    Enable {
        #[clap(long, value_parser)]
        lhost: String,

        #[clap(long, value_parser, default_value_t = 9001)]
        lport: u16
    }
}
