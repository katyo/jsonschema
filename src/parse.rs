use crate::{Error, Result};
use std::{path::Path, str::FromStr};

macro_rules! decl_formats {
    ($(
        $(#[$attr:meta])*
        $type:ident $name:ident [ $($ext:literal),* ];
    )*) => {
        $(
            $(#[$attr])*
            mod $name;
        )*

        /// Supported input data formats
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Format {
            $(
                $(#[$attr])*
                $type,
            )*
        }

        impl FromStr for Format {
            type Err = &'static str;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                use Format::*;
                Ok(match s {
                    $(
                        $(#[$attr])*
                        stringify!($name) => $type,
                    )*
                    _ => return Err("unknown"),
                })
            }
        }

        impl Format {
            /// List of all variants
            pub const LIST: &'static [&'static str] = &[
                $(
                    $(#[$attr])*
                    stringify!($name),
                )*
            ];

            /// Determining data format from file name
            pub fn from_path(path: &Path) -> Option<Self> {
                use Format::*;
                let ext = path.extension()?;
                $(
                    $(#[$attr])*
                    if $(ext == $ext ||)* false {
                        return Some($type);
                    }
                )*
                None
            }

            /// Unified data parsing
            pub fn parse_data(&self, topic: &str, path: &Path, data: &[u8]) -> Option<json::Value> {
                use Format::*;

                match self {
                    $(
                        $(#[$attr])*
                        $type => {
                            let data = $name::from_slice::<$name::Value>(data)
                                .map_err(|error| {
                                    log::error!(
                                        "Unable to parse {} {} from '{}' due to: {}",
                                        stringify!($name),
                                        topic,
                                        path.display(),
                                        error
                                    );
                                })
                                .ok()?;
                            $name::to_json(data)
                        }
                    )*
                }
            }
        }
    };
}

decl_formats! {
    Json json ["json"];
    #[cfg(feature = "json5")]
    Json5 json5 ["json5"];
    #[cfg(feature = "yaml")]
    Yaml yaml ["yaml", "yml"];
    #[cfg(feature = "toml")]
    Toml toml ["toml"];
    #[cfg(feature = "ron")]
    Ron ron ["ron"];
    #[cfg(feature = "bson")]
    Bson bson ["bson"];
    #[cfg(feature = "cbor")]
    Cbor cbor ["cbor"];
    #[cfg(feature = "pickle")]
    Pickle pickle ["pickle"];
}

pub fn parse_json(topic: &str, path: &Path, data: &Vec<u8>) -> Result<::json::Value> {
    ::json::from_slice(data).map_err(|error| {
        log::warn!(
            "Unable to parse {} from '{}' due to: {}",
            topic,
            path.display(),
            error
        );
        Error::Parse
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_from_path() {
        use Format::*;

        assert_eq!(
            Format::from_path(Path::new("path/to/data.json")),
            Some(Json)
        );
        #[cfg(feature = "yaml")]
        assert_eq!(
            Format::from_path(Path::new("path/to/data.yaml")),
            Some(Yaml)
        );
        #[cfg(feature = "yaml")]
        assert_eq!(Format::from_path(Path::new("path/to/data.yml")), Some(Yaml));
        #[cfg(not(feature = "yaml"))]
        assert_eq!(Format::from_path(Path::new("path/to/data.yml")), None);
        #[cfg(feature = "toml")]
        assert_eq!(
            Format::from_path(Path::new("path/to/data.toml")),
            Some(Toml)
        );
        #[cfg(not(feature = "toml"))]
        assert_eq!(Format::from_path(Path::new("path/to/data.toml")), None);
        assert_eq!(Format::from_path(Path::new("path/to/dir")), None);
    }
}
