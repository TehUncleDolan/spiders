use super::{
    selectors::CHAPTER_SELECTOR,
    series,
};
use crate::{
    utils,
    Chapter,
    Error,
    Result,
    Series,
};
use kuchiki::traits::*;
use once_cell::unsync::Lazy;
use regex::Regex;
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
        .map(|link| {
            let url = url_from_element(&link)?;
            let (id, volume) = parse_title(&link)?;

            Ok(Chapter {
                id,
                series,
                volume,
                url,
            })
        })
        .collect::<Result<Vec<_>>>()
}

/// Get a path to the directory where where the chapter will be saved.
pub(super) fn get_path(path: &Path, chapter: &Chapter<'_>) -> PathBuf {
    // If volume is known, chapter will be stored in the volume's directory.
    // Otherwise, chapter will be saved in its own directory.
    let dirname = if let Some(volume) = chapter.volume.as_ref() {
        format!("{} {:03}", chapter.series.title, volume)
    } else {
        let chapter_id = utils::format_chapter_id(chapter.id);
        format!("{} {:03}", chapter.series.title, chapter_id)
    };
    let dirname = crate::fs::sanitize_name(&dirname);
    let path = series::get_path(path, chapter.series);

    [path, dirname].iter().collect()
}

/// Extract chapter ID and volume name (if any) from chapter's title.
#[allow(clippy::expect_used)] // Regexp is hardcoded and valid.
fn parse_title(
    element: &kuchiki::ElementData,
) -> Result<(f64, Option<String>)> {
    let extract_info = Lazy::new(|| {
        Regex::new(
            r#"(?i)(?:Vol.(?P<volume>\d+) )?Chapter (?P<id>\d+(?:\.\d+)?)"#,
        )
        .expect("invalid chapter regexp")
    });

    let attributes = element.attributes.borrow();
    let title = attributes
        .get("title")
        .ok_or_else(|| Error::Scraping("chapter title not found".to_owned()))?;

    let matches = extract_info.captures(&title).ok_or_else(|| {
        Error::Scraping(format!("cannot match on chapter title: {}", title))
    })?;

    let volume = matches
        .name("volume")
        .map(|m| format!("{:0>2}", m.as_str()));
    let id = matches
        .name("id")
        .expect("invalid capture group for chapter ID")
        .as_str()
        .parse::<f64>()
        .map_err(|err| {
            Error::Scraping(format!("invalid chapter ID: {}", err))
        })?;

    Ok((id, volume))
}

/// Extract the chapter URL.
#[allow(clippy::filter_next)]
fn url_from_element(element: &kuchiki::ElementData) -> Result<Url> {
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

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(0, 0),
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/mangakakalot.com/series.html");
        let html = std::fs::read_to_string(&path).expect("test data");
        let document = kuchiki::parse_html().one(html);

        let chapters = scrape_from_html(&document, &series).unwrap();

        assert_eq!(chapters.len(), 330);
    }
}

// }}}
