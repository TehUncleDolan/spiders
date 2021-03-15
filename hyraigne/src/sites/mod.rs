//! Provides web scrapers for dedicated websites.

mod traits;
mod webtoons;

pub use traits::Site;
pub use webtoons::Site as Webtoons;
