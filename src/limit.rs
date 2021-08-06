//! Gets limit from `docker.io`'s ratelimitpreview manifest

use super::err::{DrlErr, DrlResult, ExitCode};
use super::token::Token;
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use std::fmt;
use std::str::FromStr;

const LIMIT_URL: &str = "https://registry-1.docker.io/v2/ratelimitpreview/test/manifests/latest";

/// The current state of the rate limit
#[derive(Debug, Default, Copy, Clone)]
pub struct Limit {
    /// Number of remaining requests of the rate limit, out of `total`
    pub remaining: usize,
    /// Total number of possible requests for the rate limit
    pub total: usize,
}

impl fmt::Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.remaining, self.total)
    }
}

/// Parse the named header `key` from `headers`.
///
/// # Errors
///
/// An error is returned if there is no header with the passed key, or if the value of the header
/// cannot be parsed as a `T`
fn parse_header<T: FromStr>(headers: &HeaderMap, key: &str) -> DrlResult<T>
where
    T::Err: fmt::Display,
{
    let header = headers
        .get(key)
        .ok_or_else(|| DrlErr::new("error parsing rate limit".into(), ExitCode::Parsing))?;

    let value = header.to_str().map_err(|e| {
        DrlErr::new(
            format!("error parsing rate limit: {}", e),
            ExitCode::Parsing,
        )
    })?;

    // Take up to the first semicolon, or the end
    let end = value.find(';').unwrap_or(value.len());
    let value = &value[..end];

    T::from_str(value).map_err(|e| {
        DrlErr::new(
            format!("error parsing rate limit: {}", e),
            ExitCode::Parsing,
        )
    })
}

/// Gets rate limit from `docker.io`
///
/// # Arguments
///
/// * `t` - `Token` JWT token from `docker.io`
pub async fn get_limit(t: &Token) -> DrlResult<Limit> {
    let client = Client::new();
    let req = client.get(LIMIT_URL);
    let req = req.bearer_auth(t.token.as_str());

    // send request
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("failed to connect to docker.io: {}", e);
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    // check for over limit status code
    match resp.status() {
        StatusCode::OK => (),
        StatusCode::TOO_MANY_REQUESTS => {
            let msg = String::from("over limit");
            let err = DrlErr::new(msg, ExitCode::OverLimit);
            return Err(err);
        }
        _ => {
            let msg = format!("error connecting to docker.io: {}", resp.status());
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    // limits stored in the headers
    let headers = resp.headers();

    // get rate limit
    let total: usize = parse_header(headers, "ratelimit-limit")?;
    let remaining: usize = parse_header(headers, "ratelimit-remaining")?;

    Ok(Limit { remaining, total })
}
