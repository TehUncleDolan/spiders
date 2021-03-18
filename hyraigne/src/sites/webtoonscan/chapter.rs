use super::{
    selectors::{
        CHAPTER_SELECTOR,
        CHAPTER_URL_SELECTOR,
    },
    series,
};
use crate::{
    Chapter,
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
            let url = url_from_html(chapter)?;
            let id = f64::from(id_from_url(&url)?);

            Ok(Chapter { id, series, url })
        })
        .collect::<Result<Vec<_>>>()
}

/// Get a path to the directory where where the chapter will be saved.
pub(super) fn get_path(path: &Path, chapter: &Chapter<'_>) -> PathBuf {
    let dirname = crate::fs::sanitize_name(&format!(
        "{} {:03}",
        chapter.series.title, chapter.id
    ));
    let path = series::get_path(path, chapter.series);

    [path, dirname].iter().collect()
}

/// Extract chapter ID from chapter's URL.
fn id_from_url(url: &Url) -> Result<u16> {
    Path::new(url.path())
        .file_name()
        .ok_or_else(|| Error::Scraping("chapter ID not found".to_owned()))?
        .to_str()
        .ok_or_else(|| {
            Error::Scraping("invalid UTF-8 in chapter ID".to_owned())
        })?
        .parse::<u16>()
        .map_err(|err| Error::Scraping(format!("invalid chapter ID: {}", err)))
}

/// Extract the chapter URL.
#[allow(clippy::filter_next)]
fn url_from_html(html: &kuchiki::NodeRef) -> Result<Url> {
    let link = CHAPTER_URL_SELECTOR
        .filter(html.descendants().elements())
        .next()
        .ok_or_else(|| Error::Scraping("chapter link not found".to_owned()))?;

    let url = link
        .attributes
        .borrow()
        .get("href")
        .ok_or_else(|| Error::Scraping("chapter URL not found".to_owned()))?
        .to_owned();

    Url::parse(&url).map_err(|err| {
        Error::Scraping(format!("invalid chapter URL `{}`: {}", url, err))
    })
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Pagination;

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(0, 0),
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/webtoonscan.com/series.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let chapters = scrape_from_html(&document, &series).unwrap();

        assert_eq!(chapters.len(), 79);
    }
}

// }}}