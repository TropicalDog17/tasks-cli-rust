use std::env;
use std::collections::HashMap;
use task_cli::result::Result;
use task_cli::cmd::{create, import, Opts};
#[derive(Debug)]
pub struct Cli{
    opts: Opts,
    cmd: Cmd
}

impl Cli{
    pub fn parse() -> Result<Cli>{
        // Only support arguments.
        let args: Vec<String> = env::args().collect();
        let cmd = args.get(1);

        let cli: Cli = match cmd.unwrap().as_str(){
            "create" => {
                let due: Option<String> = match args.len(){
                    4 => None,
                    5 => Some(args[4].to_owned()),
                    _ => unimplemented!()
                };
                let opts = Opts::builder().name(Some(args[2].to_owned())).desc(Some(args[3].to_owned())).due(due).create()?;
                Cli{opts, cmd: Cmd::Import}
            },
            _ => unimplemented!()
        };
        Ok(cli)
    }
}
#[derive(Debug)]
pub enum Cmd{
    Create,
    Import,
    Export
}
fn main() -> Result {
    let cli = Cli::parse()?;
    dbg!(cli);
    Ok(())
    // run(cli)
}
fn run(cli: Cli) -> Result{
    Ok(())
}