/*!

[jsonschema](https://docs.rs/jsonschema) validator support

*/

use super::{Error, Path, Result, Standard};
use jsonschema::{Draft, JSONSchema, CompilationOptions};

pub struct CompiledSchema<'c> {
    schema: JSONSchema,
    _phantom: core::marker::PhantomData<&'c char>,
}

impl<'c> CompiledSchema<'c> {
    pub fn compile(schema: &'c json::Value, std: Option<Standard>) -> Result<Self> {
        let mut opts = CompilationOptions::default();

        if let Some(std) = std {
            opts.with_draft(conv_std(std));
        }

        opts.compile(schema)
            .map_err(|error| {
                log::error!("Unable to compile JSON Schema due to: {}", error);
                Error::Compile
            })
            .map(|schema| Self { schema, _phantom: core::marker::PhantomData })
    }

    pub fn validate(&self, path: &Path, data: &json::Value, quiet: bool) -> Result<u32> {
        Ok(if quiet {
            u32::from(!self.schema.is_valid(data))
        } else if let Err(errors) = self.schema.validate(data) {
            println!("Data is not valid");
            #[allow(clippy::suspicious_map)]
            {
                errors
                    .map(|error| {
                        println!("{}: {}", path.display(), error);
                    })
                    .count() as u32
            }
        } else {
            println!("Data is valid");
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
