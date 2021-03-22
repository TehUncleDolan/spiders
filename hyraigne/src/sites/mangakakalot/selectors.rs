//! CSS selectors to scrape `https://mangakakalot.com`.

use once_cell::sync::Lazy;

/// Select the series title.
#[allow(clippy::expect_used)]
pub(super) static SERIES_TITLE_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile(".manga-info-text h1")
            .expect("invalid series title selector")
    });

/// Select `<meta property="og:url" content="URL" />`
#[allow(clippy::expect_used)]
pub(super) static SERIES_URL_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile("meta[property=\"og:url\"]")
            .expect("invalid series URL selector")
    });

/// Select chapter links in the chapter list.
#[allow(clippy::expect_used)]
pub(super) static CHAPTER_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile(".chapter-list .row a")
            .expect("invalid chapter selector")
    });

/// Select image URLs from the chapter page.
#[allow(clippy::expect_used)]
pub(super) static PAGE_URL_SELECTOR: Lazy<kuchiki::Selectors> =
    Lazy::new(|| {
        kuchiki::Selectors::compile(".container-chapter-reader img")
            .expect("invalid page URL selector")
    });
