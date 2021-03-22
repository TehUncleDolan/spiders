use super::selectors::{
    SERIES_TITLE_SELECTOR,
    SERIES_URL_SELECTOR,
};
use crate::{
    types::Pagination,
    Error,
    Result,
    Series,
};
use kuchiki::traits::*;
use std::path::{
    Path,
    PathBuf,
};
use url::Url;

/// Scrape series info from the given HTML.
pub(super) fn scrape_from_html(html: &kuchiki::NodeRef) -> Result<Series> {
    Ok(Series {
        title: title_from_html(&html)?,
        url: url_from_html(&html)?,
        // No pagination here, everything is listed on the first page.
        pagination: Pagination::new(0, 0),
    })
}

/// Get a path to the directory where where the series will be saved.
pub(super) fn get_path(path: &Path, series: &Series) -> PathBuf {
    let dirname = crate::fs::sanitize_name(&series.title);

    [path, &dirname].iter().collect()
}

/// Extract series title from the content of `<div class="post-title">`.
#[allow(clippy::filter_next)]
fn title_from_html(html: &kuchiki::NodeRef) -> Result<String> {
    let raw_title = SERIES_TITLE_SELECTOR
        .filter(html.descendants().elements())
        .next()
        .ok_or_else(|| Error::Scraping("series title not found".to_owned()))?
        .text_contents();
    let title = raw_title.trim();

    if title.is_empty() {
        return Err(Error::Scraping("series title is missing".to_owned()));
    }
    Ok(title.to_owned())
}

/// Extract series URL from `<meta property="og:url" content="URL" />`
#[allow(clippy::filter_next)]
fn url_from_html(html: &kuchiki::NodeRef) -> Result<Url> {
    let element = SERIES_URL_SELECTOR
        .filter(html.descendants().elements())
        .next()
        .ok_or_else(|| Error::Scraping("series URL not found".to_owned()))?;
    let attributes = element.attributes.borrow();
    let url = attributes
        .get("content")
        .ok_or_else(|| Error::Scraping("series URL is missing".to_owned()))?;

    Url::parse(url).map_err(|err| {
        Error::Scraping(format!("invalid series URL `{}`: {}", url, err))
    })
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraping() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/webtoonscan.com/series.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let series = scrape_from_html(&document).unwrap();

        assert_eq!(series.title, "Legend Of The Northern Blade");
        assert_eq!(
            series.url.as_str(),
            "https://webtoonscan.com/manhwa/legend-of-the-northern-blade/"
        );
        assert_eq!(series.pagination.chapter_count, 0);
        assert_eq!(series.pagination.page_size, 0);
    }
}

// }}}
