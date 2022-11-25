use clap::Parser;

mod operator;
mod share;

#[tokio::main]
async fn main() {
    let cli = operator::args::Cli::parse();
    let (mut opr, notifications) = operator::Operator::new(cli.server, cli.rootca, cli.name)
        .await
        .unwrap();
    let mut console = share::Console::new();
    operator::add_commands(&mut console);
    console.cli_loop(&mut opr, notifications);
}
