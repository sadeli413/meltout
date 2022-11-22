pub mod parsers;

use super::commands::{Commander, Handler};
use super::errors::Error;
use crate::share::operatorpb::Notification;
use rustyline::Editor;
use std::collections::HashMap;
use std::io::Write;
use tokio::sync::mpsc::Receiver;
use tonic::Status;

// A CLI struct for both the server and operator
pub struct Console<T: Commander> {
    pub commands: HashMap<String, Handler<T>>,
}

impl<T> Console<T>
where
    T: Commander,
{
    // Create a console containing a hashmap of commands
    pub fn new() -> Console<T> {
        Console {
            commands: HashMap::new(),
        }
    }

    // Accept user input
    pub fn cli_loop(
        &self,
        meltout: &mut T,
        notifications_rx: Receiver<Result<Notification, Status>>,
    ) {
        // TODO: set a console root home to store history
        let history = "history.txt";
        let mut rl = rustyline::Editor::<()>::new().unwrap();
        let _ = rl.load_history(history);

        tokio::spawn(async move {
            notification_loop(notifications_rx).await;
        });

        loop {
            match self.get_input(&mut rl, meltout) {
                Ok(_) => (),
                Err(Error::ReadlineErr(_)) => break,
                Err(Error::CommandParsingErr(e)) => eprintln!("{}", e.render().ansi()),
                Err(err) => eprintln!("{}", err),
            }
        }
        rl.save_history(history).unwrap();
    }

    // Get one line of user input
    fn get_input(&self, rl: &mut Editor<()>, meltout: &mut T) -> Result<(), Error> {
        // TODO: Change the prompt to reflect the state of the program
        let line = rl
            .readline(">> ")
            .map_err(|e| Error::ReadlineErr(e.to_string()))?;
        rl.add_history_entry(&line);

        let line = shlex::split(&line)
            .ok_or_else(|| Error::InputErr("Could not parse input.".to_string()))?;

        let line = line.iter().map(|s| s.as_str()).collect();

        self.parse_cmd(meltout, line)?;

        Ok(())
    }

    // Execute the command
    fn parse_cmd(&self, meltout: &mut T, line: Vec<&str>) -> Result<(), Error> {
        let cmd = line.get(0).ok_or_else(|| Error::CommandNotFound)?;

        let handler = self
            .commands
            .get(&cmd.to_string())
            .ok_or_else(|| Error::CommandNotFound)?;

        handler(meltout, line)?;

        Ok(())
    }
}

async fn notification_loop(mut notifications_rx: Receiver<Result<Notification, Status>>) {
    while let Some(notification) = notifications_rx.recv().await {
        if let Err(e) = display_notification(notification) {
            eprintln!("{}", e);
        }
    }
}

fn display_notification(notification: Result<Notification, Status>) -> Result<(), Error> {
    let n = notification.map_err(|e| Error::RpcError(e))?;
    std::io::stdout()
        .write_all(&n.stdout)
        .map_err(|e| Error::IOErr(e))?;
    std::io::stderr()
        .write_all(&n.stderr)
        .map_err(|e| Error::IOErr(e))?;
    Ok(())
}
