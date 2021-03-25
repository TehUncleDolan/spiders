use super::Chapter;
use crate::utils;
use std::path::{
    Path,
    PathBuf,
};
use url::Url;

/// A page.
pub struct Page<'a> {
    /// Chapter containing this page.
    pub(crate) chapter: &'a Chapter<'a>,

    /// URL of the page.
    pub(crate) main: Url,

    /// Fallback URL (used if the first one doesn't work), if any.
    pub(crate) fallback: Option<Url>,
}

impl Page<'_> {
    /// Get the file path of the page on disk.
    pub(crate) fn path(&self, basedir: &Path, index: usize) -> PathBuf {
        let dirpath = self.chapter.path(basedir);
        let extension = crate::fs::extname_from_url(&self.main);
        // If we store inside the volume directory, we need to prefix with the
        // chapter ID to avoid name collisions.
        let filename = if self.chapter.volume.is_some() {
            let chapter_id = utils::format_chapter_id(self.chapter.id);
            format!("{:03}-{:03}.{}", chapter_id, index, extension)
        } else {
            format!("{:03}.{}", index, extension)
        };

        [&dirpath, Path::new(&filename)].iter().collect()
    }
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
    fn test_page_path_with_volume() {
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

        let path = page.path(Path::new("Downloads"), 42);

        assert_eq!(path, PathBuf::from(expected));
    }

    #[test]
    fn test_page_path_without_volume() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(0, 0),
        };
        let chapter = Chapter {
            id: 30.0,
            series: &series,
            volume: None,
            url: Url::parse("http://example.com/30/").unwrap(),
        };
        let page = Page {
            chapter: &chapter,
            main: Url::parse("http://example.com/10/uWu.jpg").unwrap(),
            fallback: None,
        };
        let expected = "Downloads/Example/Example 030/042.jpg";

        let path = page.path(Path::new("Downloads"), 42);

        assert_eq!(path, PathBuf::from(expected));
    }
}

// }}}
