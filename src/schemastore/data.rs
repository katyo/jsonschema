use crate::Uri;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Schema list
#[derive(Clone, Serialize, Deserialize)]
pub struct SchemaList {
    /// JSON schema
    #[serde(rename = "$schema")]
    pub schema: String,

    /// Version
    pub version: f32,

    /// List of schemas
    pub schemas: Vec<SchemaInfo>,
}

impl Display for SchemaList {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for schema in &self.schemas {
            schema.fmt(f)?;
        }
        Ok(())
    }
}

/// Schema info
#[derive(Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    /// Schema name
    pub name: String,

    /// Schema content url
    pub url: Wrapped<Uri>,

    /// Schema description
    pub description: String,

    /// Glob pattern for files matched to schema
    #[serde(rename = "fileMatch")]
    pub file_match: Option<Vec<String>>,

    /// Other versions of schema
    pub versions: Option<HashMap<String, Wrapped<Uri>>>,
}

impl Display for SchemaInfo {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        "- name: ".fmt(f)?;
        self.name.fmt(f)?;
        "\n  description: ".fmt(f)?;
        self.description.fmt(f)?;
        "\n  url: ".fmt(f)?;
        self.url.fmt(f)?;
        if let Some(file_match) = &self.file_match {
            "\n  file_match:".fmt(f)?;
            for file_match in file_match {
                "\n    - ".fmt(f)?;
                file_match.fmt(f)?;
            }
        }
        if let Some(versions) = &self.versions {
            "\n  versions:".fmt(f)?;
            for (version, url) in versions {
                "\n    ".fmt(f)?;
                version.fmt(f)?;
                ": ".fmt(f)?;
                url.fmt(f)?;
            }
        }
        "\n".fmt(f)
    }
}

/// Wrapped string-like data
#[serde_with::serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct Wrapped<T>(
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(bound = "T: Display + FromStr, T::Err: Display")]
    T,
);

impl<T> AsRef<T> for Wrapped<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Wrapped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Wrapped<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Display for Wrapped<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}
