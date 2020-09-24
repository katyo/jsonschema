/*!

Parsing input data with support for multiple text and binary formats

*/

use std::path::Path;

macro_rules! decl_formats {
    ($(
        $(#[$attr:meta])*
        $type:ident $name:ident [ $($ext:literal),* ];
    )*) => {
        // Define modules
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

        impl std::str::FromStr for Format {
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

        impl std::fmt::Display for Format {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type => stringify!($name).fmt(f),
                    )*
                }
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
                let ext = path.extension()?;
                $(
                    $(#[$attr])*
                    if $(ext == $ext ||)* false {
                        return Some(Self::$type);
                    }
                )*
                None
            }

            /// Unified data parsing
            pub fn parse_data(&self, topic: &str, path: &Path, data: &[u8]) -> Option<json::Value> {
                match self {
                    $(
                        $(#[$attr])*
                        Self::$type => {
                            $name::from_slice(data)
                                .map_err(|error| {
                                    log::error!(
                                        "Unable to parse {} {} from '{}' due to: {}",
                                        stringify!($name),
                                        topic,
                                        path.display(),
                                        error
                                    );
                                })
                                .ok()
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

    #[cfg(feature = "json5")]
    #[test]
    fn parse_json5() {
        let data: json::Value = json5::from_slice(b"{a:1, b:false, c:[1,true,'a',], }").unwrap();
        println!("json5: {:?}", data);
        assert_eq!(
            data,
            ::json::json!(
                {
                    "a": 1,
                    "b": false,
                    "c": [1, true, "a"]
                }
            )
        );
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn parse_yaml() {
        let data: json::Value =
            yaml::from_slice(b"a: 1\nb: false\nc:\n- 1\n- true\n- a\n").unwrap();
        println!("yaml: {:?}", data);
        assert_eq!(
            data,
            ::json::json!(
                {
                    "a": 1,
                    "b": false,
                    "c": [1, true, "a"]
                }
            )
        );
    }

    #[cfg(feature = "toml")]
    #[test]
    fn parse_toml() {
        let data: json::Value = toml::from_slice(b"a = 1\nb = false\nc = [1, true, 'a',]").unwrap();
        println!("toml: {:?}", data);
        assert_eq!(
            data,
            ::json::json!(
                {
                    "a": 1,
                    "b": false,
                    "c": [1, true, "a"]
                }
            )
        );
    }

    #[cfg(feature = "toml")]
    #[test]
    fn parse_toml_nested() {
        let data: json::Value =
            toml::from_slice(b"[topic]\na = 1\nb = false\nc = [1, true, 'a',]").unwrap();
        println!("toml: {:?}", data);
        assert_eq!(
            data,
            ::json::json!(
                {
                    "topic": {
                        "a": 1,
                        "b": false,
                        "c": [1, true, "a"]
                    }
                }
            )
        );
    }
}
