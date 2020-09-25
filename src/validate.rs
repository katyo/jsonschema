/*!

Validating JSON-like data with support for multiple validators

*/

pub(self) use crate::{Error, Result};
pub(self) use std::path::Path;

macro_rules! decl_standards {
    ($(
        $(#[$attr:meta])*
        $type:ident $name:ident;
    )*) => {
        /// JSON Schema standard
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Standard {
            $(
                $(#[$attr])*
                $type,
            )*
        }

        impl std::str::FromStr for Standard {
            type Err = &'static str;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $(#[$attr])*
                        stringify!($name) => Self::$type,
                    )*
                    _ => return Err("unknown"),
                })
            }
        }

        impl std::fmt::Display for Standard {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type => stringify!($name).fmt(f),
                    )*
                }
            }
        }

        impl Standard {
            /// List of all variants
            pub const LIST: &'static [&'static str] = &[
                $(
                    $(#[$attr])*
                    stringify!($name),
                )*
            ];
        }
    };
}

macro_rules! decl_validators {
    ($(
        $(#[$attr:meta])*
        $type:ident $name:ident;
    )*) => {
        // Define modules
        $(
            $(#[$attr])*
            mod $name;
        )*

        /// Supported validators
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Validator {
            $(
                $(#[$attr])*
                $type,
            )*
        }

        impl std::str::FromStr for Validator {
            type Err = &'static str;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $(#[$attr])*
                        stringify!($name) => Self::$type,
                    )*
                    _ => return Err("unknown"),
                })
            }
        }

        impl std::fmt::Display for Validator {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type => stringify!($name).fmt(f),
                    )*
                }
            }
        }

        impl Validator {
            /// List of all variants
            pub const LIST: &'static [&'static str] = &[
                $(
                    $(#[$attr])*
                    stringify!($name),
                )*
            ];

            /// Compile JSON schema
            pub fn compile_schema<'c>(&self, schema: &'c json::Value, std: Option<Standard>) -> Result<CompiledSchema<'c>> {
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type => $name::CompiledSchema::compile(schema, std).map(CompiledSchema::$type),
                    )*
                }
            }
        }

        /// Compiled schema
        pub enum CompiledSchema<'c> {
            $(
                $(#[$attr])*
                $type($name::CompiledSchema<'c>),
            )*
        }

        impl<'c> CompiledSchema<'c> {
            /// Validate JSON data
            pub fn validate_data(&self, path: &Path, data: &json::Value, quiet: bool) -> Result<u32> {
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type(compiled_schema) => compiled_schema.validate(path, data, quiet),
                    )*
                }
            }
        }
    };
}

decl_standards! {
    Draft4 darft4;
    Draft6 draft6;
    Draft7 draft7;
}

decl_validators! {
    #[cfg(feature = "valico")]
    Valico valico;
    #[cfg(feature = "jsonschema")]
    JsonSchema jsonschema;
    #[cfg(feature = "jsonschema-valid")]
    JsonSchemaValid jsonschema_valid;
}
