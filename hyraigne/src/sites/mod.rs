//! Provides web scrapers for dedicated websites.

mod traits;
mod webtoons;
mod webtoonscan;

pub use traits::Site;
pub use webtoons::Site as Webtoons;
pub use webtoonscan::Site as WebtoonScan;
