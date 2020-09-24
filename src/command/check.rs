use super::{utils, Args, CmdResult, Error, Format, Path, PathBuf, State, StructOpt};

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Data format
    #[structopt(short, long, default_value = "json", possible_values = Format::LIST)]
    pub format: Format,

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
                        "Unable to determine {} format of '{}'",
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

        let schema = jsonschema::JSONSchema::compile(&schema, None).map_err(|error| {
            log::error!("Unable to compile JSON Schema due to: {}", error);
            Error::Compile
        })?;

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
        schema: &jsonschema::JSONSchema,
        path: &Path,
        input: &mut dyn std::io::Read,
    ) -> CmdResult {
        let data = utils::read_input(topic, path, input)?;
        let format = Format::from_path(&path);
        let format = if let Some(format) = format {
            format
        } else {
            log::error!(
                "Unable to determine {} format of '{}'. Try using {} by default.",
                topic,
                path.display(),
                self.format
            );
            self.format
        };
        let data = format.parse_data(topic, path, &data).ok_or(Error::Parse)?;
        Ok(Self::check(args, schema, path, &data))
    }

    fn check(args: &Args, schema: &jsonschema::JSONSchema, path: &Path, data: &json::Value) -> u32 {
        if args.verbose {
            if let Err(errors) = schema.validate(data) {
                println!("Data is not valid");
                let mut n = 0;
                for error in errors {
                    n += 1;
                    eprintln!("{} {}", path.display(), error)
                }
                n
            } else {
                println!("Data is valid");
                0
            }
        } else {
            if schema.is_valid(data) {
                0
            } else {
                1
            }
        }
    }
}
