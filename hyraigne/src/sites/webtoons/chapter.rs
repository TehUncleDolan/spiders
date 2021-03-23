use super::selectors::{
    CHAPTER_SELECTOR,
    CHAPTER_URL_SELECTOR,
};
use crate::{
    Chapter,
    Error,
    Result,
    Series,
};
use kuchiki::traits::*;
use url::Url;

/// Extract every chapter listed in the given HTML.
#[allow(clippy::filter_map)]
pub(super) fn scrape_from_html<'a>(
    html: &kuchiki::NodeRef,
    series: &'a Series,
) -> Result<Vec<Chapter<'a>>> {
    CHAPTER_SELECTOR
        .filter(html.descendants().elements())
        .map(|chapter| {
            let chapter = chapter.as_node();

            Ok(Chapter {
                id: f64::from(id_from_html(chapter)?),
                series,
                volume: None,
                url: url_from_html(chapter)?,
            })
        })
        .collect::<Result<Vec<_>>>()
}

/// Extract chapter ID from `<li id="episode_82" data-episode-no="ID">`.
pub(super) fn id_from_html(html: &kuchiki::NodeRef) -> Result<u16> {
    html.as_element()
        .ok_or_else(|| {
            Error::Scraping(
                "expected chapter node to be an element data".to_owned(),
            )
        })?
        .attributes
        .borrow()
        .get("data-episode-no")
        .ok_or_else(|| Error::Scraping("chapter ID not found".to_owned()))?
        .parse::<u16>()
        .map_err(|err| Error::Scraping(format!("invalid chapter ID: {}", err)))
}

/// Extract the chapter URL.
#[allow(clippy::filter_next)]
fn url_from_html(html: &kuchiki::NodeRef) -> Result<Url> {
    let element = CHAPTER_URL_SELECTOR
        .filter(html.descendants().elements())
        .next()
        .ok_or_else(|| Error::Scraping("chapter link not found".to_owned()))?;
    let attributes = element.attributes.borrow();

    let url = attributes
        .get("href")
        .ok_or_else(|| Error::Scraping("chapter URL not found".to_owned()))?;

    Url::parse(url).map_err(|err| {
        Error::Scraping(format!("invalid chapter URL `{}`: {}", url, err))
    })
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Pagination;
    use std::path::PathBuf;

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(78, 10),
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/webtoons.com/series.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let chapters = scrape_from_html(&document, &series).unwrap();

        assert_eq!(chapters.len(), 10);
    }
}

// }}}
