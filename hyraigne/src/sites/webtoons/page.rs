use super::selectors::PAGE_URL_SELECTOR;
use crate::{
    Chapter,
    Error,
    Page,
    Result,
};
use kuchiki::traits::*;
use url::Url;

/// Scrape page links from the chapter's page HTML.
#[allow(clippy::filter_map)]
pub(super) fn scrape_from_html<'a>(
    html: &kuchiki::NodeRef,
    chapter: &'a Chapter<'_>,
) -> Result<Vec<Page<'a>>> {
    PAGE_URL_SELECTOR
        .filter(html.descendants().elements())
        .map(|node| {
            let attributes = node.attributes.borrow();
            let url = attributes.get("data-url").ok_or_else(|| {
                Error::Scraping("page URL not found".to_owned())
            })?;

            let url = Url::parse(url).map_err(|err| {
                Error::Scraping(format!("invalid page URL `{}`: {}", url, err))
            })?;

            Ok(Page {
                chapter,
                main: url,
                fallback: None,
            })
        })
        .collect::<Result<Vec<_>>>()
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        types::Pagination,
        Series,
    };
    use std::path::PathBuf;

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(1, 0),
        };
        let chapter = Chapter {
            id: 10.0,
            series: &series,
            volume: None,
            url: Url::parse("http://example.com/10/").unwrap(),
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/webtoons.com/chapter.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let pages = scrape_from_html(&document, &chapter).unwrap();

        assert_eq!(pages.len(), 32);
    }
}

// }}}
