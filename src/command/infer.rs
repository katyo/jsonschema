use super::{utils, Args, CmdResult, Error, Format, Path, PathBuf, State, StructOpt};

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Input data format
    #[structopt(short, long, possible_values = Format::LIST)]
    pub format: Option<Format>,

    /// Pretty formatted output
    #[structopt(short, long)]
    pub pretty: bool,

    /// Input data file name
    #[structopt()]
    pub input: Option<PathBuf>,

    /// Output JSON Schema file name
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
}

impl Command {
    pub fn run(&self, args: &Args, _state: &State) -> CmdResult {
        let topic = "data";
        if let Some(path) = &self.input {
            let mut file = utils::open_file(topic, path)?;
            self.read_and_infer(args, topic, path, &mut file)
        } else {
            let path = Path::new("stdin");
            let mut file = std::io::stdin();
            self.read_and_infer(args, topic, path, &mut file)
        }
    }

    fn read_and_infer(
        &self,
        args: &Args,
        topic: &str,
        path: &Path,
        input: &mut dyn std::io::Read,
    ) -> CmdResult {
        let data = utils::read_input(topic, path, input)?;
        let format = self
            .format
            .or_else(|| Format::from_path(path))
            .ok_or_else(|| {
                log::error!(
                    "Format of {} from '{}' is not given and cannot to be inferred from filename. Try use -f option to set it.",
                    topic,
                    path.display()
                );
                Error::Parse
            })?;

        let data = format.parse_data(topic, path, &data).ok_or(Error::Parse)?;

        let schema = infers::JSONSchema::new(&data);
        //let schema = schema.detect_format(true);
        let schema = schema.infer();

        let topic = "JSON Schema";
        let contents = utils::format_json(topic, &schema, self.pretty)?;

        if let Some(path) = &self.output {
            log::info!("Saving {} to file '{}'...", topic, path.display());

            args.check_output_file(path)?;
            let mut file = utils::create_file(topic, path)?;
            utils::write_output(topic, path, &mut file, &contents)?;
        } else {
            let path = Path::new("stdout");
            let mut file = std::io::stdout();
            utils::write_output(topic, path, &mut file, &contents)?;
        }

        Ok(0)
    }
}
