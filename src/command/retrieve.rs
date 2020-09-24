use super::{utils, Args, CmdResult, Error, Path, PathBuf, State, StructOpt};

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Pretty formatted output
    #[structopt(short, long)]
    pub pretty: bool,

    /// Pattern (keyword or regexp)
    #[structopt()]
    pub pattern: String,

    /// Output file name
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
}

impl Command {
    pub fn run(&self, args: &Args, state: &State) -> CmdResult {
        let contents = state
            .schema_store
            .get_one(&[&self.pattern], true, false)
            .ok_or(Error::Query)?
            .1;

        let topic = "JSON Schema";
        let contents = utils::format_json(topic, &contents, self.pretty)?;

        if let Some(path) = &self.output {
            log::info!("Saving {} to file '{}'...", topic, path.display());

            args.check_output_file(&path)?;
            let mut file = utils::create_file(topic, &path)?;
            utils::write_output(topic, &path, &mut file, &contents)?;
        } else {
            let path = Path::new("stdout");
            let mut file = std::io::stdout();
            utils::write_output(topic, &path, &mut file, &contents)?;
        }

        Ok(0)
    }
}
