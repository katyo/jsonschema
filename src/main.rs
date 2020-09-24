mod cache;
mod command;
mod error;
mod parse;
pub mod utils;

#[cfg(any(
    feature = "jsonschema",
    feature = "jsonschema-valid",
    feature = "valico"
))]
mod validate;

#[cfg(feature = "schemastore")]
mod schemastore;

#[cfg(feature = "http_req")]
mod http;

#[cfg(feature = "http_req")]
pub use http_req::uri::Uri;

#[cfg(feature = "schemastore")]
pub use schemastore::SchemaStore;

pub use cache::Cache;
pub use command::Args;
pub use error::Error;
pub use parse::Format;

#[cfg(any(
    feature = "jsonschema",
    feature = "jsonschema-valid",
    feature = "valico"
))]
pub use validate::{CompiledSchema, Standard, Validator};

pub type Result<T> = std::result::Result<T, Error>;

pub struct State {
    #[cfg(feature = "schemastore")]
    pub schema_store: SchemaStore,
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
