use super::{utils, Args, CmdResult, Error, Path, PathBuf, State, StructOpt};

#[derive(StructOpt, Debug)]
pub struct Command {
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

        let contents = json::to_string_pretty(&contents).map_err(|error| {
            log::error!("Unable to format JSON due to: {}", error);
            Error::Format
        })?;

        let topic = "JSON Schema";

        if let Some(path) = &self.output {
            log::info!("Saving {} to file '{}'...", topic, path.display());

            args.check_output_file(&path)?;
            let mut file = utils::create_file(topic, &path)?;
            utils::write_output(topic, &path, &mut file, &contents.as_bytes())?;
        } else {
            let path = Path::new("stdout");
            let mut file = std::io::stdout();
            utils::write_output(topic, &path, &mut file, &contents.as_bytes())?;
        }

        Ok(0)
    }
}
