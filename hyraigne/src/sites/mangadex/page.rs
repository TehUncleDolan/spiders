use super::models::{
    ChapterDetail,
    Response,
};
use crate::{
    Chapter,
    Error,
    Page,
    Result,
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
}

// }}}
