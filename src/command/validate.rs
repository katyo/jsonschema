use super::{
    utils, Args, CmdResult, CompiledSchema, Error, Format, Path, PathBuf, Standard, State,
    StructOpt, Validator,
};

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Input data format
    #[structopt(short, long, possible_values = Format::LIST)]
    pub format: Option<Format>,

    /// Using standard
    #[structopt(short, long, possible_values = Standard::LIST)]
    pub standard: Option<Standard>,

    /// Using validator
    #[structopt(short, long, default_value = Validator::LIST[0], possible_values = Validator::LIST)]
    pub validator: Validator,

    #[cfg(feature = "schemastore")]
    /// Schema file or name
    #[structopt()]
    pub schema: PathBuf,

    #[cfg(not(feature = "schemastore"))]
    /// Schema file
    #[structopt()]
    pub schema: PathBuf,

    /// Data files to validate (otherwise data will be read from stdin)
    #[structopt()]
    pub input: Vec<PathBuf>,
}

impl Command {
    pub fn run(&self, args: &Args, state: &State) -> CmdResult {
        let schema = if self.schema.is_file() {
            let topic = "JSON Schema";
            let path = &self.schema;
            let mut file = utils::open_file(topic, path)?;
            let data = utils::read_input(topic, path, &mut file)?;

            Format::from_path(&self.schema)
                .ok_or_else(|| {
                    log::error!(
                        "Unable to determine {} format from '{}'",
                        topic,
                        path.display()
                    );
                    Error::Parse
                })?
                .parse_data(topic, path, &data)
                .ok_or(Error::Parse)?
        } else {
            let pattern = self.schema.display().to_string();
            state
                .schema_store
                .get_one(&[&pattern], true, false)
                .ok_or(Error::Query)?
                .1
        };

        let schema = self.validator.compile_schema(&schema, self.standard)?;

        let topic = "data";

        if self.input.len() > 0 {
            let mut errors = 0u32;
            for path in &self.input {
                if !path.is_file() {
                    log::error!("Input {} file '{}' not found", topic, path.display());
                    return Err(Error::Open);
                }
                let mut file = utils::open_file(topic, path)?;
                errors += self.read_parse_check(args, topic, &schema, path, &mut file)?;
            }
            Ok(errors)
        } else {
            let path = Path::new("stdin");
            let mut file = std::io::stdin();
            self.read_parse_check(args, topic, &schema, &path, &mut file)
        }
    }

    fn read_parse_check(
        &self,
        args: &Args,
        topic: &str,
        schema: &CompiledSchema,
        path: &Path,
        input: &mut dyn std::io::Read,
    ) -> CmdResult {
        let data = utils::read_input(topic, path, input)?;
        let format = self
            .format
            .or_else(|| Format::from_path(&path))
            .ok_or_else(|| {
                log::error!(
                    "Format of {} from '{}' is not given and cannot to be inferred from filename. Try use -f option to set it.",
                    topic,
                    path.display()
                );
                Error::Parse
            })?;
        let data = format.parse_data(topic, path, &data).ok_or(Error::Parse)?;
        schema.validate_data(path, &data, args.quiet)
    }
}
