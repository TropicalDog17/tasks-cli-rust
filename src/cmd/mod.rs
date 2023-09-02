use crate::result::Result;
pub mod import;
pub mod create;

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Opts{
    pub name: String,
    pub desc: String,
    pub due: String,
    pub file: String
}
impl Opts{
    // This method will help users to discover the builder
    pub fn builder() -> OptsBuilder {
        OptsBuilder::default()
    }
}
#[derive(Debug, Default)]
pub struct OptsBuilder {
    // Probably lots of optional fields.
    name: Option<String>,
    desc: Option<String>,
    due: Option<String>,
    file: Option<String>
}
impl OptsBuilder{
    pub fn new() -> OptsBuilder{
        OptsBuilder { ..Default::default() }
    }
    pub fn name(mut self, name: Option<String>) -> OptsBuilder{
        self.name = name;
        self
    }
    pub fn desc(mut self, desc: Option<String>) -> OptsBuilder{
        self.desc = desc;
        self
    }
    pub fn due(mut self, due: Option<String>) -> OptsBuilder{
        self.due = due;
        self
    }
    pub fn file(mut self, file: Option<String>) -> OptsBuilder{
        self.file = file;
        self
    }
    pub fn create(self) -> Result<Opts>{
        let mut default_opts = Opts::default();
        if let Some(file_path) = &self.file{
            default_opts.file = file_path.to_owned();
        };
        if self.due.is_some(){
            default_opts.due = self.due.unwrap();
        }
        if self.name.is_some() && self.desc.is_some(){
            default_opts.name = self.name.unwrap();
            default_opts.desc = self.desc.unwrap();
        }
        Ok(default_opts)
    }
}
