use super::{
    chapter,
    selectors::PAGE_URL_SELECTOR,
};
use crate::{
    Chapter,
    Error,
    Page,
    Result,
};
use kuchiki::traits::*;
use std::path::{
    Path,
    PathBuf,
};
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
            let url = node
                .attributes
                .borrow()
                .get("data-url")
                .ok_or_else(|| {
                    Error::Scraping("page URL not found".to_owned())
                })?
                .to_owned();

            let url = Url::parse(&url).map_err(|err| {
                Error::Scraping(format!("invalid page URL `{}`: {}", url, err))
            })?;

            Ok(Page { chapter, main: url })
        })
        .collect::<Result<Vec<_>>>()
}

/// Get the file path of the page on disk.
pub(super) fn get_path(path: &Path, page: &Page<'_>, index: usize) -> PathBuf {
    let dirpath = chapter::get_path(path, page.chapter);
    let extension = crate::fs::extname_from_url(&page.main);
    let filename = format!("{:03}.{}", index, extension);

    [&dirpath, Path::new(&filename)].iter().collect()
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        types::Pagination,
        Series,
    };

    #[test]
    fn test_get_path() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(78, 10),
        };
        let chapter = Chapter {
            id: 10.0,
            series: &series,
            url: Url::parse("http://example.com/10/").unwrap(),
        };
        let page = Page {
            chapter: &chapter,
            main: Url::parse("http://example.com/10/uWu.jpg").unwrap(),
        };
        let expected = "Downloads/Example/Example 010/042.jpg";

        let path = get_path(Path::new("Downloads"), &page, 42);

        assert_eq!(path, PathBuf::from(expected));
    }

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
