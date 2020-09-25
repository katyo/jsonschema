/*!

[jsonschema](https://docs.rs/jsonschema) validator support

*/

use super::{Error, Path, Result, Standard};
use jsonschema::{Draft, JSONSchema};

pub struct CompiledSchema<'c> {
    schema: JSONSchema<'c>,
}

impl<'c> CompiledSchema<'c> {
    pub fn compile(schema: &'c json::Value, std: Option<Standard>) -> Result<Self> {
        JSONSchema::compile(&schema, std.map(conv_std))
            .map_err(|error| {
                log::error!("Unable to compile JSON Schema due to: {}", error);
                Error::Compile
            })
            .map(|schema| Self { schema })
    }

    pub fn validate(&self, path: &Path, data: &json::Value, quiet: bool) -> Result<u32> {
        Ok(if quiet {
            if self.schema.is_valid(data) {
                0
            } else {
                1
            }
        } else {
            if let Err(errors) = self.schema.validate(data) {
                println!("Data is not valid");
                errors
                    .map(|error| {
                        println!("{}: {}", path.display(), error);
                    })
                    .count() as u32
            } else {
                println!("Data is valid");
                0
            }
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
