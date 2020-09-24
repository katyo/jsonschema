use super::{Args, CmdResult, PathBuf, State, StructOpt};

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Input data file name
    #[structopt()]
    pub input: Option<PathBuf>,

    /// Output JSON Schema file name
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
}

impl Command {
    pub fn run(&self, args: &Args, state: &State) -> CmdResult {
        Ok(0)
    }
}
