/*!

Command-line arguments and command processing

*/

pub(self) use std::path::{Path, PathBuf};
pub(self) use structopt::StructOpt;

pub(self) use crate::{utils, Error, Result, State};

#[cfg(any(
    feature = "jsonschema",
    feature = "jsonschema-valid",
    feature = "valico",
))]
pub(self) use crate::{CompiledSchema, Standard, Validator};

#[cfg(any(
    feature = "jsonschema",
    feature = "jsonschema-valid",
    feature = "valico",
    feature = "infers",
))]
pub(self) use crate::Format;

/// Command result
pub type CmdResult = Result<u32>;

#[cfg(feature = "http_req")]
pub(self) use crate::Uri;

/// Logging levels list
const LOG_LEVELS: &[&str] = &["error", "warn", "info", "debug", "trace"];

macro_rules! decl_commands {
    ($(
        $(#[$attr:meta])*
        $type:ident $name:ident;
    )*) => {
        // Declare modules
        $(
            $(#[$attr])*
            mod $name;
        )*

        /// Supported sub-commands
        #[derive(StructOpt, Debug)]
        pub enum Command {
            $(
                $(#[$attr])*
                $type($name::Command),
            )*
        }

        impl Command {
            /// Run sub-command
            pub fn run(&self, args: &Args, state: &State) -> CmdResult {
                #[allow(unused_doc_comments)]
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type(cmd_args) => cmd_args.run(args, state),
                    )*
                }
            }
        }
    };
}

decl_commands! {
    /// Search schemas on schema store
    #[cfg(feature = "schemastore")]
    Search search;

    /// Retrieve schema contents
    #[cfg(feature = "schemastore")]
    Retrieve retrieve;

    /// Infer schema from data
    #[cfg(feature = "infers")]
    Infer infer;

    /// Validate data using json schema
    #[cfg(any(
        feature = "jsonschema",
        feature = "jsonschema-valid",
        feature = "valico"
    ))]
    Validate validate;
}

/// Command-line arguments
#[derive(StructOpt, Debug)]
pub struct Args {
    /// Logging level
    #[structopt(short = "l", long, env, default_value = "warn", possible_values = LOG_LEVELS)]
    pub log_level: String,

    /// Verbose mode
    #[structopt(short, long)]
    pub verbose: bool,

    /// Quiet mode
    #[structopt(short, long)]
    pub quiet: bool,

    /// Force overwrite
    #[structopt(short = "F", long)]
    pub force: bool,

    #[cfg(feature = "cache")]
    /// Cache directory
    #[structopt(short = "c", long, env)]
    pub cache_dir: Option<PathBuf>,

    #[cfg(feature = "cache")]
    /// Disable caching
    #[structopt(short = "x", long, env)]
    pub no_cache: bool,

    #[cfg(feature = "schemastore")]
    /// Schema store catalog url
    #[structopt(
        short = "U",
        long,
        env,
        default_value = "https://www.schemastore.org/api/json/catalog.json"
    )]
    pub catalog_url: Uri,

    /// Command to execute
    #[structopt(subcommand)]
    pub command: Command,
}

impl Args {
    /// Run program
    pub fn run(&self, state: &State) -> CmdResult {
        self.command.run(self, state)
    }

    /// Check output filepath to prevent unwanted overwriting
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
