//! CSS selectors to scrape `https://www.webtoons.com`.

use once_cell::sync::Lazy;

/// Select `<meta property="og:title" content="TITLE" />`
#[allow(clippy::expect_used)]
pub(super) static SERIES_TITLE_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile("meta[property=\"og:title\"]")
            .expect("invalid series title selector")
    });

/// Select `<meta property="og:url" content="URL" />`
#[allow(clippy::expect_used)]
pub(super) static SERIES_URL_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile("meta[property=\"og:url\"]")
            .expect("invalid series URL selector")
    });

/// Select chapter entries in the chapter list.
#[allow(clippy::expect_used)]
pub(super) static CHAPTER_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile("#_listUl li")
            .expect("invalid chapter selector")
    });

/// Select chapter link in the chapter entry.
#[allow(clippy::expect_used)]
pub(super) static CHAPTER_URL_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile("a").expect("invalid chapter URL selector")
    });

/// Select image URLs from the chapter page.
#[allow(clippy::expect_used)]
pub(super) static PAGE_URL_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile("#_imageList img")
            .expect("invalid page URL selector")
    });
