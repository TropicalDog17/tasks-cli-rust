use super::Opts;
use std::fs::File;
use std::io::Write;
use crate::result::Result;
use crate::task::{Task, TaskList};
pub struct Cmd{}
impl Cmd{
    pub fn run(opts: Opts, tl: TaskList) -> Result{
        export_tasklist(&opts.file, tl) 
    }
}
pub fn export_tasklist(path: &str, tl: TaskList) -> Result{
    let j = serde_json::to_string(&tl)?;
    let mut file = File::create(path)?;
    file.write_all(j.as_bytes())?;
    Ok(())    
}
#[cfg(test)]
mod tests{
    use super::*;
    use crate::cmd::export::Cmd;
    use std::path::Path;
    #[test]
    fn test_write_sample(){
        let path =  "src/sample/foo.txt";
        let mut tl = TaskList::default();
        let t = Task::create("Test task", "Test description", Some("22/12/2002"));
        tl.tasks.push(t);
        let opts = Opts::builder().file(Some("src/sample/foo.txt".to_string())).create().unwrap();
        let _ = Cmd::run(opts, tl);

        let b: bool = Path::new(path).exists();
        assert!(b);
        // std::fs::remove_file(path);
    }
}