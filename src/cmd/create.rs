use super::Opts;
use crate::result::Result;
use crate::task::Task;
pub struct Cmd{}
impl Cmd{
    pub fn run(opts: Opts) -> Result<Task>
    {
     // Get info from opts
        Ok(Task::create(&opts.name, &opts.desc, Some(&opts.due)))  
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_create_task(){
        let name = String::from("Sample task");
        let desc = String::from("Sample description");
        let task = Task::create(&name, &desc, Some(""));
        let opts = Opts::builder().name(Some(name)).desc(Some(desc)).create().unwrap();
        assert_eq!(Cmd::run(opts).unwrap(), task);
    }
}