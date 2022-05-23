/*!

[valico](https://docs.rs/valico) JSON Schema validator support

*/

use super::{Error, Path, Result, Standard};
use valico::json_schema::Scope;

pub struct CompiledSchema<'c> {
    scope: Scope,
    url: url::Url,
    _phantom: core::marker::PhantomData<&'c char>,
}

impl<'c> CompiledSchema<'c> {
    pub fn compile(schema: &'c json::Value, _std: Option<Standard>) -> Result<Self> {
        let mut scope = Scope::new();
        scope
            .compile(schema.clone(), false)
            .map_err(|error| {
                log::error!("Unable to compile JSON Schema due to: {}", error);
                Error::Compile
            })
            .map(|url| Self {
                scope,
                url,
                _phantom: core::marker::PhantomData,
            })
    }

    pub fn validate(&self, path: &Path, data: &json::Value, quiet: bool) -> Result<u32> {
        let schema = self.scope.resolve(&self.url).ok_or_else(|| {
            log::error!(
                "Unable to resolve previously compiled valico JSON Schema: {}",
                self.url
            );
            Error::Query
        })?;
        let result = schema.validate(data);

        Ok(if result.errors.is_empty() {
            if !quiet {
                println!("Data is valid");
            }
            0
        } else if !quiet {
            println!("Data is not valid");
            for error in &result.errors {
                eprintln!("{} {}", path.display(), error)
            }
            result.errors.len() as u32
        } else {
            1
        })
    }
}
