// From file format, import to a list of Task

use crate::task::{TaskList, read_file};
use crate::result::Result; 
use super::Opts;


pub struct Cmd{}
impl Cmd{
    pub fn run(opts: Opts) -> Result<TaskList>{
        let data = read_file(&opts.file);
        let tl: TaskList = serde_json::from_str(&data)?;
        Ok(tl)
    }
}