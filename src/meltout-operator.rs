mod operator;
mod share;

#[tokio::main]
async fn main() {
    let (mut opr, notifications) = operator::Operator::new("https://127.0.0.1:8001".to_string())
        .await
        .unwrap();
    let mut console = share::Console::new();
    operator::add_commands(&mut console);
    console.cli_loop(&mut opr, notifications);
}
