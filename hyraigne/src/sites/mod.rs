//! Provides web scrapers for dedicated websites.

use crate::Options;
use url::Url;

mod mangadex;
mod traits;
mod webtoons;
mod webtoonscan;

pub use traits::Site;

use mangadex::Site as MangaDex;
use webtoons::Site as Webtoons;
use webtoonscan::Site as WebtoonScan;

/// Return a web spider adapted to the given URL.
///
/// If the given URL is not supported, `None` is returned.
#[must_use]
pub fn get_spider_for(url: &Url, opts: Options) -> Option<Box<dyn Site>> {
    url.host_str().and_then(|hostname| {
        let spider: Option<Box<dyn Site>> = match hostname {
            "mangadex.org" => Some(Box::new(MangaDex::new(opts))),
            "www.webtoons.com" => Some(Box::new(Webtoons::new(opts))),
            "webtoonscan.com" => Some(Box::new(WebtoonScan::new(opts))),
            _ => None,
        };

        spider
    })
}
