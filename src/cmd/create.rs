use super::Opts;
use crate::result::Result;
use crate::task::Task;
pub struct Cmd{}
impl Cmd{
    pub fn run(opts: Opts) -> Result<Task>
    {
     // Get info from opts
        Ok(Task::create(opts.name, opts.desc, opts.due))  
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_create_task(){
        
        let name = String::from("Sample task");
        let desc = String::from("Sample description");
        let due: Option<String> = None;
        let task = Task::create(name.clone(), desc.clone(), due.clone());
        let opts = Opts{name, desc, due};
        assert_eq!(Cmd::run(opts).unwrap(), task);
    }
}