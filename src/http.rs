/*!

Embedded HTTP client with support for redirects and caching

*/

use crate::Cache;
use http_req::{error::Error, request::Request, response::Response, uri::Uri};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    convert::TryInto,
    time::{Duration, SystemTime},
};

const REDIRECT_LIMIT: u32 = 5; // five times
const UPDATE_INTERVAL: Duration = Duration::from_secs(5 * 60); // five minutes

/// HTTP document with modified date and etag
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doc<T> {
    pub etag: Option<String>,
    pub date: Option<String>,
    pub time: SystemTime,
    pub body: T,
}

impl<T> Doc<T> {
    pub fn new(body: T, etag: Option<String>, date: Option<String>) -> Self {
        let time = SystemTime::now();
        Self {
            body,
            etag,
            date,
            time,
        }
    }

    pub fn refresh(&mut self) {
        self.time = SystemTime::now();
    }
}

pub fn get_cached<T>(cache: &Cache, url: impl AsRef<str>) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    let url = url.as_ref();

    let cached_doc: Option<Doc<T>> = cache.get(&url);
    if let Some(cached_doc) = cached_doc {
        if cached_doc.time.elapsed().unwrap_or(UPDATE_INTERVAL) < UPDATE_INTERVAL {
            // prevent too often requests
            return Some(cached_doc.body);
        }
        match check(url, &cached_doc.etag, &cached_doc.date) {
            // when error happened or not modified
            None | Some(false) => {
                let mut cached_doc = cached_doc;

                cached_doc.refresh();
                cache.put(&url, &cached_doc);

                return Some(cached_doc.body);
            }
            _ => (),
        }
    }

    let doc = get(&url)?;

    cache.put(&url, &doc);

    Some(doc.body)
}

/// Fetch from HTTP
pub fn get<T>(url: impl AsRef<str>) -> Option<Doc<T>>
where
    T: DeserializeOwned,
{
    let url = url.as_ref();
    let mut body = Vec::new();

    log::info!("Starting HTTP GET request to: {}", url);

    let res = with_redirects(url, |url| {
        body.clear();
        Request::new(url).send(&mut body)
    })?;

    log::info!(
        "Received HTTP GET response from: {} with status: {}",
        url,
        res.status_code(),
    );

    if res.status_code().is_success() {
        let etag = res.headers().get("ETag").cloned();
        let date = res.headers().get("Last-Modified").cloned();

        let body = json::from_slice(&body)
            .map_err(|error| {
                log::error!(
                    "Unable to parse json document from '{}' due to: {}",
                    url,
                    error
                );
            })
            .ok()?;

        return Some(Doc::new(body, etag, date));
    }

    log::warn!("Unexpected HTTP response: {}", res.status_code());

    None
}

/// Check HTTP resource for updates
pub fn check(
    url: impl AsRef<str>,
    etag: &Option<impl AsRef<str>>,
    date: &Option<impl AsRef<str>>,
) -> Option<bool> {
    let url = url.as_ref();
    let mut body = Vec::new();

    log::info!("Starting HTTP HEAD request to: {}", url);

    let res = with_redirects(url, |url| {
        body.clear();
        let mut req = Request::new(url);
        if let Some(etag) = etag {
            req.header("If-None-Match", etag.as_ref());
        }
        if let Some(date) = date {
            req.header("If-Modified-Since", date.as_ref());
        }
        req.send(&mut body)
    })?;

    log::info!(
        "Received HTTP HEAD response from: {} with status: {}",
        url,
        res.status_code(),
    );

    if res.status_code().is(|i| i == 304) {
        // not modified
        return Some(false);
    } else if res.status_code().is_success() {
        // modified
        return Some(true);
    }

    log::warn!("Unexpected HTTP response: {}", res.status_code());

    None
}

fn with_redirects<F: FnMut(&Uri) -> Result<Response, Error>>(
    url: &str,
    mut mk_req: F,
) -> Option<Response> {
    let mut right_redirects = REDIRECT_LIMIT;

    let mut url = url.to_string();

    loop {
        let urlp = url
            .as_str()
            .try_into()
            .map_err(|error| {
                if right_redirects < REDIRECT_LIMIT {
                    log::warn!("Invalid redirect url '{}' due to: {}", url, error)
                } else {
                    log::warn!("Invalid url '{}' due to: {}", url, error)
                }
            })
            .ok()?;

        let res = mk_req(&urlp)
            .map_err(|error| {
                log::warn!("Invalid request '{}' due to: {}", url.to_string(), error);
            })
            .ok()?;

        if res.status_code().is_redirect() {
            if right_redirects == 0 {
                break;
            }
        } else {
            return Some(res);
        }

        right_redirects -= 1;

        if let Some(location) = res.headers().get("Location") {
            url = location.into();
        } else {
            break;
        }
    }

    None
}
