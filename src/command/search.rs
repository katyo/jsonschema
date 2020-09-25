use super::{Args, CmdResult, Error, State, StructOpt};

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Patterns (keywords or regexps)
    #[structopt()]
    pub patterns: Vec<String>,

    /// Including descriptions
    #[structopt(short = "d", long)]
    pub with_descriptions: bool,
}

impl Command {
    pub fn run(&self, args: &Args, state: &State) -> CmdResult {
        let list = if self.patterns.len() == 0 {
            state.schema_store.list().map(|list| list.schemas)
        } else {
            state
                .schema_store
                .find(&self.patterns, true, self.with_descriptions)
        }
        .ok_or(Error::Query)?;

        if !args.quiet {
            println!("Found {} schemas", list.len());
        }

        if args.verbose {
            for schema in &list {
                println!("{}", schema);
            }
        } else if !args.quiet {
            for schema in &list {
                println!("- {}", schema.name);
            }
        }

        Ok(list.len() as u32)
    }
}
