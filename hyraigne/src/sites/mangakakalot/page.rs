use super::{
    chapter,
    selectors::PAGE_URL_SELECTOR,
};
use crate::{
    utils,
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
            let attributes = node.attributes.borrow();
            let url = attributes.get("src").ok_or_else(|| {
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

/// Get the file path of the page on disk.
pub(super) fn get_path(path: &Path, page: &Page<'_>, index: usize) -> PathBuf {
    let dirpath = chapter::get_path(path, page.chapter);
    let extension = crate::fs::extname_from_url(&page.main);
    // If we store inside the volume directory, we need to prefix with the
    // chapter ID to avoid name collisions.
    let filename = if page.chapter.volume.is_some() {
        let chapter_id = utils::format_chapter_id(page.chapter.id);
        format!("{:03}-{:03}.{}", chapter_id, index, extension)
    } else {
        format!("{:03}.{}", index, extension)
    };

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
            pagination: Pagination::new(64, u16::MAX),
        };
        let chapter = Chapter {
            id: 42.0,
            series: &series,
            volume: None,
            url: Url::parse("http://example.com/42/").unwrap(),
        };
        let page = Page {
            chapter: &chapter,
            main: Url::parse("http://example.com/42/uWu.jpg").unwrap(),
            fallback: None,
        };
        let expected = "Downloads/Example/Example 042/007.jpg";

        let path = get_path(Path::new("Downloads"), &page, 7);

        assert_eq!(path, PathBuf::from(expected));
    }

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(64, u16::MAX),
        };
        let chapter = Chapter {
            id: 42.0,
            series: &series,
            volume: None,
            url: Url::parse("http://example.com/42/").unwrap(),
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/mangakakalot.com/chapter.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let pages = scrape_from_html(&document, &chapter).unwrap();

        assert_eq!(pages.len(), 23);
    }
}

// }}}
