mod args;
mod cache;
mod error;
mod parse;
mod utils;

#[cfg(feature = "schemastore")]
mod schemastore;

#[cfg(feature = "http_req")]
mod http;

use std::path::Path;

#[cfg(feature = "http_req")]
pub use http_req::uri::Uri;

#[cfg(feature = "schemastore")]
pub use schemastore::SchemaStore;

pub use args::Args;
pub use cache::Cache;
pub use error::Error;
pub use parse::Format;

use args::{CheckArgs, Command, RetrieveArgs, SearchArgs};

pub type Result<T> = std::result::Result<T, Error>;

pub type CmdResult = Result<u32>;

pub struct State {
    #[cfg(feature = "schemastore")]
    pub schema_store: SchemaStore,
}

impl SearchArgs {
    pub fn run(&self, args: &Args, state: &State) -> CmdResult {
        let list = if self.patterns.len() == 0 {
            state.schema_store.list().map(|list| list.schemas)
        } else {
            state
                .schema_store
                .find(&self.patterns, true, self.with_descriptions)
        }
        .ok_or(Error::Query)?;

        println!("Found {} schemas", list.len());

        if args.verbose {
            for schema in list {
                println!("{}", schema);
            }
        } else {
            for schema in list {
                println!("- {}", schema.name);
            }
        }
        Ok(0)
    }
}

impl RetrieveArgs {
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

impl CheckArgs {
    pub fn run(&self, args: &Args, state: &State) -> CmdResult {
        let schema = if self.schema.is_file() {
            let topic = "JSON Schema";
            let path = &self.schema;
            let mut file = utils::open_file(topic, path)?;
            let data = utils::read_input(topic, path, &mut file)?;
            parse::parse_json(topic, path, &data)?
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
                errors += Self::read_parse_check(args, topic, &schema, path, &mut file)?;
            }
            Ok(errors)
        } else {
            let path = Path::new("stdin");
            let mut file = std::io::stdin();
            Self::read_parse_check(args, topic, &schema, &path, &mut file)
        }
    }

    fn read_parse_check(
        args: &Args,
        topic: &str,
        schema: &jsonschema::JSONSchema,
        path: &Path,
        input: &mut dyn std::io::Read,
    ) -> CmdResult {
        let data = utils::read_input(topic, path, input)?;
        let data = parse::parse_json(topic, path, &data)?;
        Ok(Self::check(args, schema, path, &data))
    }

    fn check(args: &Args, schema: &jsonschema::JSONSchema, path: &Path, data: &json::Value) -> u32 {
        if args.verbose {
            if let Err(errors) = schema.validate(data) {
                println!("Data is not valid");
                let mut n = 0;
                for error in errors {
                    n += 1;
                    eprintln!("{}{}", path.display(), error)
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

impl Args {
    pub fn run(&self, state: &State) -> CmdResult {
        use Command::*;
        match &self.command {
            #[cfg(feature = "schemastore")]
            Search(cmd_args) => cmd_args.run(self, state),
            #[cfg(feature = "schemastore")]
            Retrieve(cmd_args) => cmd_args.run(self, state),
            Check(cmd_args) => cmd_args.run(self, state),
        }
    }

    pub fn check_output_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if path.is_dir() {
            log::error!(
                "Output path '{}' is directory and cannot be overwitten in any case",
                path.display()
            );
            return Err(Error::Conflict);
        } else if path.is_file() && !self.force {
            log::error!(
                "Output file '{}' already exists and wont be overwritten. Use -f option to force overwriting",
                path.display()
            );
            return Err(Error::Conflict);
        }
        Ok(())
    }
}

#[paw::main]
fn main(mut args: Args) {
    std::env::set_var("LOG_LEVEL", &args.log_level);
    pretty_env_logger::init_custom_env("LOG_LEVEL");

    //log::debug!("Args: {:?}", args);

    args.fix_cache();

    let state = State {
        #[cfg(feature = "schemastore")]
        schema_store: SchemaStore::new(&args),
    };

    std::process::exit(match args.run(&state) {
        Ok(n) => n as i32,
        Err(e) => e as i32,
    });
}
