use std::path::PathBuf;
use structopt::StructOpt;

#[cfg(feature = "http_req")]
use crate::{Format, Uri};

static LOG_LEVELS: &[&str] = &["error", "warn", "info", "debug", "trace"];

#[derive(StructOpt, Debug)]
pub struct SearchArgs {
    /// Patterns (keywords or regexps)
    #[structopt()]
    pub patterns: Vec<String>,

    /// Including descriptions
    #[structopt(short = "d", long)]
    pub with_descriptions: bool,
}

#[derive(StructOpt, Debug)]
pub struct RetrieveArgs {
    /// Pattern (keyword or regexp)
    #[structopt()]
    pub pattern: String,

    /// Output file name
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub struct CheckArgs {
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

#[derive(StructOpt, Debug)]
pub enum Command {
    #[cfg(feature = "schemastore")]
    /// Search schamas on schema store
    Search(SearchArgs),

    #[cfg(feature = "schemastore")]
    /// Retrieve schema contents
    Retrieve(RetrieveArgs),

    /// Validate data using json schama
    Check(CheckArgs),
}

#[derive(StructOpt, Debug)]
pub struct Args {
    /// Logging level
    #[structopt(short = "l", long, env, default_value = "warn", possible_values = LOG_LEVELS)]
    pub log_level: String,

    /// Verbose output
    #[structopt(short, long)]
    pub verbose: bool,

    /// Force overwrite
    #[structopt(short, long)]
    pub force: bool,

    #[cfg(feature = "cache")]
    /// Cache directory
    #[structopt(short = "c", long, env)]
    pub cache_dir: Option<PathBuf>,

    #[cfg(feature = "cache")]
    /// Disable caching
    #[structopt(short = "n", long, env)]
    pub no_cache: bool,

    #[cfg(feature = "schemastore")]
    /// Schema store catalog url
    #[structopt(
        short = "u",
        long,
        env,
        default_value = "https://www.schemastore.org/api/json/catalog.json"
    )]
    pub catalog_url: Uri,

    /// Command to execute
    #[structopt(subcommand)]
    pub command: Command,
}
