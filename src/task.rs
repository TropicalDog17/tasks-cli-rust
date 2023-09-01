use serde::{Deserialize, Serialize};
use serde_json::Result;

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
#[derive(Debug,Default, Serialize, Deserialize)]
pub struct TaskList{
    tasks: Vec<Task>,
    categories: Vec<String>,
}
impl TaskList{
   
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::*;
    #[test]
    fn test_import_from_raw(){
        let data = r#"
            {
                "name": "Do homework",
                "desc": "Test description"
            }
        "#;
        let t: Task = serde_json::from_str(data).unwrap();
        assert_eq!(t.name, "Do homework".to_string());
        assert_eq!(t.desc, "Test description".to_string());


    }
}