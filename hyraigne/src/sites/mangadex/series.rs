use super::{
    models::{
        Response,
        Series,
    },
    API_BASE_URL,
};
use crate::{
    types::Pagination,
    Error,
    Result,
};
use std::path::{
    Path,
    PathBuf,
};
use url::Url;

/// Extract series metadata from the API response.
pub(super) fn extract_from_response(
    response: Response<Series>,
) -> Result<crate::Series> {
    let endpoint = format!("{}/manga/{}", API_BASE_URL, response.data.id);

    Ok(crate::Series {
        title: response.data.title,
        url: Url::parse(&endpoint).map_err(|err| {
            Error::Scraping(format!(
                "invalid series endpoint {}: {}",
                endpoint, err
            ))
        })?,
        pagination: Pagination::new(0, 0),
    })
}

/// Get a path to the directory where where the series will be saved.
pub(super) fn get_path(path: &Path, series: &crate::Series) -> PathBuf {
    let dirname = crate::fs::sanitize_name(&series.title);

    [path, &dirname].iter().collect()
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraping() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/mangadex.org/series.json");
        let json = std::fs::read_to_string(&path).expect("test data");
        let response = serde_json::from_str(&json).expect("invalid JSON");

        let series = extract_from_response(response).unwrap();

        assert_eq!(series.title, "Kingdom");
        assert_eq!(
            series.url.as_str(),
            "https://api.mangadex.org/v2/manga/642"
        );
        assert_eq!(series.pagination.chapter_count, 0);
        assert_eq!(series.pagination.page_size, 0);
    }
}

// }}}
