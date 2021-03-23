use super::{
    chapter,
    selectors::{
        CHAPTER_SELECTOR,
        SERIES_TITLE_SELECTOR,
        SERIES_URL_SELECTOR,
    },
};
use crate::{
    types::Pagination,
    Error,
    Result,
    Series,
};
use kuchiki::traits::*;
use url::Url;

/// Scrape series info from the given HTML.
pub(super) fn scrape_from_html(html: &kuchiki::NodeRef) -> Result<Series> {
    Ok(Series {
        title: title_from_html(&html)?,
        url: url_from_html(&html)?,
        pagination: pagination_from_html(&html)?,
    })
}

/// Extract series title from `<meta property="og:title" content="TITLE" />`
#[allow(clippy::filter_next)]
fn title_from_html(html: &kuchiki::NodeRef) -> Result<String> {
    Ok(SERIES_TITLE_SELECTOR
        .filter(html.descendants().elements())
        .next()
        .ok_or_else(|| Error::Scraping("series title not found".to_owned()))?
        .attributes
        .borrow()
        .get("content")
        .ok_or_else(|| Error::Scraping("series title is missing".to_owned()))?
        .to_owned())
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

    Url::parse(&url).map_err(|err| {
        Error::Scraping(format!("invalid series URL `{}`: {}", url, err))
    })
}

/// Infer pagination scheme from the first page of the chapter list.
#[allow(clippy::filter_map)]
fn pagination_from_html(html: &kuchiki::NodeRef) -> Result<Pagination> {
    let chapters = CHAPTER_SELECTOR
        .filter(html.descendants().elements())
        .map(|chapter| chapter::id_from_html(chapter.as_node()))
        .collect::<Result<Vec<_>>>()
        .map_err(|err| {
            Error::Scraping(format!(
                "failed to scrape chapter list to infer pagination: {}",
                err
            ))
        })?;

    assert!(!chapters.is_empty());

    #[allow(clippy::cast_possible_truncation)] // No risk here.
    Ok(Pagination::new(chapters[0], chapters.len() as u16))
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_scraping() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/webtoons.com/series.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let series = scrape_from_html(&document).unwrap();

        assert_eq!(series.title, "Hell is Other People");
        assert_eq!(series.url.as_str(), "https://www.webtoons.com/fr/thriller/hell-is-other-people/list?title_no=1841");
        assert_eq!(series.pagination.chapter_count, 78);
        assert_eq!(series.pagination.page_size, 10);
    }
}

// }}}
