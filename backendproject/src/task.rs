use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task_name: String,
    pub task_description: String,
    pub task_completed: bool,
}


pub fn save_tasks(task: Task) -> std::io::Result<()> {
    let mut file = File::create("tasks.csv")?;
    let mut writer = Writer::from_writer(file);
    writer.serialize(task)?;
    writer.flush()?;
    Ok(())
}