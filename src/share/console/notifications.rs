// Listen for notifications
use crate::share::pb::operatorpb::TaskResponse;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Notifications {
    rx: Receiver<TaskResponse>,
}

impl Notifications {
    pub fn new() -> (Notifications, Sender<TaskResponse>) {
        let (tx, rx) = channel();
        (Notifications { rx }, tx)
    }

    // Listen and print notifications
    pub fn listen_loop(&self) {
        while let Ok(msg) = self.rx.recv() {
            let stdout = std::str::from_utf8(&msg.stdout).unwrap();
            let stderr = std::str::from_utf8(&msg.stdout).unwrap();
            println!("\n---\nstdout:\n{}\nstderr:\n{}", stdout, stderr);
        }
    }
}
