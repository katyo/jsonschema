/*!

Searching and retrieving JSON Schemas from [schemastore.org](https://schemastore.org/)

*/

mod data;

use crate::{http::get_cached, Args, Cache, Uri};
use data::{SchemaInfo, SchemaList};

/// SchemaStore handle
pub struct SchemaStore {
    catalog_url: Uri,
    cache: Cache,
}

impl SchemaStore {
    /// Create new handle
    pub fn new(args: &Args) -> Self {
        let catalog_url = args.catalog_url.clone();
        let cache = Cache::open(args, "schemastore");

        Self { catalog_url, cache }
    }

    /// Get list of schemas
    pub fn list(&self) -> Option<SchemaList> {
        let url = self.catalog_url.to_string();
        get_cached(&self.cache, &url)
    }

    /// Search schemas using regex-like patterns
    pub fn find<I, S>(
        &self,
        patterns: I,
        in_names: bool,
        in_descriptions: bool,
    ) -> Option<Vec<SchemaInfo>>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let regex = build_patterns(patterns)?;
        self.list().map(|list| {
            list.schemas
                .into_iter()
                .filter(|schema| {
                    if in_names {
                        let name_matches = regex.matches(&schema.name);
                        if name_matches.matched_any() && name_matches.iter().count() == regex.len()
                        {
                            return true;
                        }
                    }
                    if in_descriptions {
                        let desc_matches = regex.matches(&schema.description);
                        if desc_matches.matched_any() && desc_matches.iter().count() == regex.len()
                        {
                            return true;
                        }
                    }
                    false
                })
                .collect()
        })
    }

    /// Search schemas using regex-like patterns
    pub fn find_one<I, S>(
        &self,
        patterns: I,
        in_names: bool,
        in_descriptions: bool,
    ) -> Option<SchemaInfo>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let schemas = self.find(patterns, in_names, in_descriptions)?;
        let count = schemas.len();
        if count == 1 {
            Some(schemas.into_iter().next().unwrap())
        } else {
            if count == 0 {
                log::error!("No schemas found.");
            } else {
                log::error!("Multiple schemas found.");
            }
            None
        }
    }

    /// Get schema by url
    pub fn get_by_url(&self, url: impl AsRef<Uri>) -> Option<json::Value> {
        let url = url.as_ref().to_string();
        get_cached(&self.cache, &url)
    }

    /// Search schemas using regex-like patterns
    pub fn get_one<I, S>(
        &self,
        patterns: I,
        in_names: bool,
        in_descriptions: bool,
    ) -> Option<(SchemaInfo, json::Value)>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let schema = self.find_one(patterns, in_names, in_descriptions)?;
        let content = self.get_by_url(&schema.url)?;
        Some((schema, content))
    }
}

fn build_patterns<I, S>(patterns: I) -> Option<regex::RegexSet>
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    regex::RegexSetBuilder::new(patterns)
        .case_insensitive(true)
        .build()
        .map_err(|error| {
            log::error!("Invalid pattern: {}", error);
        })
        .ok()
}
