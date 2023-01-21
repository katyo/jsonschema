/*!

[jsonschema-valid](https://docs.rs/jsonschema-valid) validator support

*/

use super::{Error, Path, Result, Standard};
use jsonschema_valid::{schemas::Draft, Config};

pub struct CompiledSchema<'c> {
    schema: Config<'c>,
}

impl<'c> CompiledSchema<'c> {
    pub fn compile(schema: &'c json::Value, std: Option<Standard>) -> Result<Self> {
        Config::from_schema(schema, std.map(conv_std))
            .map_err(|error| {
                log::error!("Unable to compile JSON Schema due to: {}", error);
                Error::Compile
            })
            .map(|schema| Self { schema })
    }

    pub fn validate(&self, path: &Path, data: &json::Value, quiet: bool) -> Result<u32> {
        Ok(if let Err(errors) = self.schema.validate(data) {
            if quiet {
                1
            } else {
                println!("Data is not valid");
                #[allow(clippy::suspicious_map)]
                {
                    errors
                        .map(|error| {
                            println!("{}: {}", path.display(), error);
                        })
                        .count() as u32
                }
            }
        } else {
            if !quiet {
                println!("Data is valid");
            }
            0
        })
    }
}

fn conv_std(std: Standard) -> Draft {
    match std {
        Standard::Draft4 => Draft::Draft4,
        Standard::Draft6 => Draft::Draft6,
        Standard::Draft7 => Draft::Draft7,
    }
}
