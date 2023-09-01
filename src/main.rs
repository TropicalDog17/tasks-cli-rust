use std::env;
use std::collections::HashMap;
pub struct Cli{
    opts: HashMap<String, String>,
    cmd: Cmd
}
pub enum Cmd{
    Create,
    Import,
    Export
}
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Hello, world!");
}
