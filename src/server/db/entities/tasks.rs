// TODO: Enter tasks in a database
use crate::share::implantpb;
pub struct Task {
    pub implant_id: String,            // The implant the task is made for
    pub operator_id: String,           // The operator who issued the task
    pub task: implantpb::TaskResponse, // The content of the task
}
