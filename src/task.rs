use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task{
    name: String,
    desc: String,
    due: Option<String>,

}
impl Task{
    pub fn create(name: String, desc: String, due: Option<String>) -> Self{
        Self { name, desc, due }
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskList{
    tasks: Vec<Task>,
    categories: Vec<String>,
}
impl TaskList{
   pub fn create(tasks: Vec<Task>, categories: Vec<String>) -> Self{
        Self{tasks, categories}
   }
}

pub fn read_file(file_path: &str) -> String{
    let contents =  fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}
#[cfg(test)]
mod tests{
    use super::*;
    use crate::*;
    #[test]
    fn test_import_single_task_from_file(){
        let data = read_file("src/sample/tasks.json");
        let t: Task = serde_json::from_str(&data).unwrap();
        assert_eq!(t.name, "Do homework".to_string());
        assert_eq!(t.desc, "Test description".to_string());
    }
    #[test]
    fn test_import_task_list_from_raw(){
        let data = read_file("src/sample/tasklist.json");
        let tl: TaskList = serde_json::from_str(&data).unwrap();
        assert_eq!(tl.categories.len(), 0);
        assert_eq!(tl.tasks.len(), 2);
        assert_eq!(tl.tasks.first().unwrap().name, "Do homework".to_string());
    }
}