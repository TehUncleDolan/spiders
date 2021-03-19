use super::{
    chapter,
    models::{
        ChapterDetail,
        Response,
    },
};
use crate::{
    Chapter,
    Error,
    Page,
    Result,
};
use std::path::{
    Path,
    PathBuf,
};
use url::Url;

/// Extract page links from the API response.
pub(super) fn extract_from_response<'a>(
    response: &Response<ChapterDetail>,
    chapter: &'a Chapter<'_>,
) -> Result<Vec<Page<'a>>> {
    response
        .data
        .pages
        .iter()
        .map(|page| {
            let server_url = &response.data.server;
            let fallback_url = &response.data.server_fallback;
            let path = format!("{}/{}", response.data.hash, page);

            Ok(Page {
                chapter,
                main: urljoin(server_url, &path)?,
                fallback: Some(urljoin(fallback_url, &path)?),
            })
        })
        .collect()
}

/// Get the file path of the page on disk.
pub(super) fn get_path(path: &Path, page: &Page<'_>, index: usize) -> PathBuf {
    let dirpath = chapter::get_path(path, page.chapter);
    let extension = crate::fs::extname_from_url(&page.main);
    let chapter = format_id(page.chapter.id);
    let filename = format!("{}-{:03}.{}", chapter, index, extension);

    [&dirpath, Path::new(&filename)].iter().collect()
}

/// Append `suffix` to `base` and parse the result as an URL.
fn urljoin(base: &Url, suffix: &str) -> Result<Url> {
    base.join(&suffix).map_err(|err| {
        Error::Scraping(format!(
            "failed to join {} with {}: {}",
            base.as_str(),
            suffix,
            err
        ))
    })
}

/// Format and correctly pad the chapter ID.
fn format_id(id: f64) -> String {
    let fract = id.fract();
    let width = if fract == 0.0 {
        3
    } else {
        2 + format!("{}", fract).len()
    };
    format!("{:0width$}", id, width = width)
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
            pagination: Pagination::new(0, 0),
        };
        let chapter = Chapter {
            id: 30.0,
            series: &series,
            volume: Some("10".to_owned()),
            url: Url::parse("http://example.com/30/").unwrap(),
        };
        let page = Page {
            chapter: &chapter,
            main: Url::parse("http://example.com/10/uWu.jpg").unwrap(),
            fallback: None,
        };
        let expected = "Downloads/Example/Example 10/030-042.jpg";

        let path = get_path(Path::new("Downloads"), &page, 42);

        assert_eq!(path, PathBuf::from(expected));
    }

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(0, 0),
        };
        let chapter = Chapter {
            id: 10.0,
            series: &series,
            volume: None,
            url: Url::parse("http://example.com/10/").unwrap(),
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/mangadex.org/chapter.json");
        let json = std::fs::read_to_string(&path).expect("test data");
        let response = serde_json::from_str(&json).expect("invalid JSON");

        let pages = extract_from_response(&response, &chapter).unwrap();

        assert_eq!(pages.len(), 62);
    }

    #[test]
    fn test_format_id() {
        assert_eq!(format_id(3.0), "003");
        assert_eq!(format_id(3.5), "003.5");
        assert_eq!(format_id(30.5), "030.5");
        assert_eq!(format_id(300.5), "300.5");
    }
}

// }}}
