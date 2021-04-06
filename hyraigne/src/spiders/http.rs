use crate::{
    Error,
    Result,
};
use cookie_store::CookieStore;
use kuchiki::traits::*;
use serde::de::DeserializeOwned;
use std::{
    io::Read,
    thread,
    time,
};
use url::Url;

/// A lightweight spider, built on top of a simple HTTP client.
///
/// This spider is suitable to scrape "simple" websites where all the
/// information can be extracted from the HTML served statically (without
/// executing JS code dynamically).
pub(crate) struct Spider {
    /// HTTP client.
    agent: ureq::Agent,
    /// Delay between each request.
    delay: time::Duration,
    /// Max number of retry for each request.
    retry: u8,
}

impl Spider {
    /// Initialize a new web spider.
    pub(crate) fn new(
        delay: time::Duration,
        retry: u8,
        cookie_store: Option<CookieStore>,
    ) -> Self {
        let agent = cookie_store.map_or_else(ureq::Agent::new, |store| {
            ureq::builder().cookie_store(store).build()
        });

        Self {
            agent,
            delay,
            retry,
        }
    }

    /// Retrieve and parse the page at `url`.
    pub(crate) fn get_html(&self, url: &Url) -> Result<kuchiki::NodeRef> {
        let request = self
            .agent
            .request_url("GET", url)
            .set("accept", "text/html");

        let response = self.call(&request, url)?;

        let html = response.into_string().map_err(|err| {
            log::error!("failed to read HTML from {}: {}", url.as_str(), err);
            Error::Network {
                url: url.to_string(),
            }
        })?;

        Ok(kuchiki::parse_html().one(html))
    }

    /// Download the specified page in the given buffer.
    ///
    /// Takes care of setting the referer, otherwise some websites (like
    /// webtoons.com) will block the download.
    pub(crate) fn get_image(
        &self,
        url: &Url,
        referer: &Url,
        buf: &mut Vec<u8>,
    ) -> Result<()> {
        let request = self
            .agent
            .request_url("GET", url)
            .set("accept", "image/*")
            .set("Referer", referer.as_str());

        let response = self.call(&request, url)?;

        response.into_reader().read_to_end(buf).map_err(|err| {
            log::error!("failed to read image from {}: {}", url.as_str(), err);
            Error::Network {
                url: url.to_string(),
            }
        })?;

        Ok(())
    }

    /// Make a call at `url` and parse the response.
    pub(crate) fn get_json<T>(&self, url: &Url) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let request = self
            .agent
            .request_url("GET", url)
            .set("accept", "application/json");
        let response = self.call(&request, url)?;

        serde_json::from_reader(response.into_reader()).map_err(|err| {
            log::error!("failed to read JSON from {}: {}", url.as_str(), err);
            Error::Payload {
                url: url.to_string(),
            }
        })
    }

    /// Execute a request and handle retries.
    fn call(
        &self,
        request: &ureq::Request,
        url: &Url,
    ) -> Result<ureq::Response> {
        // Wait a bit, don't overload the site.
        thread::sleep(self.delay);

        let mut i = 0;
        loop {
            i += 1;

            let res = request.clone().call();

            if let Err(ureq::Error::Status(code, ref response)) = res {
                // If we got a retryable error, we try again.
                if is_request_retryable(code) && i <= self.retry {
                    let delay = self.retry_delay(&response);

                    log::debug!(
                        "GET {} failed with status {}: retry in {} ms…",
                        url.as_str(),
                        code,
                        delay.as_millis()
                    );

                    thread::sleep(delay);
                    continue;
                }
            }

            return res.map_err(|err| {
                log::error!("HTTP request failed: {}", err);
                Error::Network {
                    url: url.to_string(),
                }
            });
        }
    }

    /// Compute the delay to wait before retrying a failed request.
    fn retry_delay(&self, response: &ureq::Response) -> time::Duration {
        response
            .header("retry-after")
            .and_then(|h| h.parse::<u64>().ok())
            .map_or(self.delay, time::Duration::from_secs)
    }
}

/// Test if request failed with a retryable error.
fn is_request_retryable(http_status: u16) -> bool {
    // 429 is Too Many Requests
    (500..=599).contains(&http_status) || http_status == 429
}
