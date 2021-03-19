/// The crate's main tyoes.
use std::{
    cmp,
    ops::RangeInclusive,
    path::PathBuf,
    time,
};
use url::Url;

/// Web spider options.
pub struct Options {
    /// Delay between each request.
    pub(crate) delay: time::Duration,

    /// Max number of retry for HTTP requests.
    pub(crate) retry: u8,

    /// Output directory.
    pub(crate) output: PathBuf,
}

impl Options {
    /// Initialize a new set of options.
    ///
    /// # Arguments
    ///
    /// * `delay`  - delay between each request (in ms)
    /// * `retry`  - max number of retry for HTTP requests
    /// * `output` - output directory, to store downloaded files.
    #[must_use]
    pub fn new(delay: u16, retry: u8, output: PathBuf) -> Self {
        let delay = time::Duration::from_millis(cmp::max(delay.into(), 10));

        Self {
            delay,
            retry,
            output,
        }
    }
}

/// Chapter filter.
pub struct Filter {
    /// Range of chapters to download.
    pub(crate) range: RangeInclusive<u16>,

    /// Chapters language.
    pub(crate) language: String,

    /// Preferred scantrad group, in case of conflict.
    pub(crate) preferred_groups: Vec<String>,
}

impl Filter {
    /// Configure a new chapter filter.
    #[must_use]
    pub fn new(
        range: RangeInclusive<u16>,
        language: Option<String>,
        preferred_groups: Vec<String>,
    ) -> Self {
        Self {
            range,
            language: language.unwrap_or_default(),
            preferred_groups,
        }
    }
}

/// A series.
pub struct Series {
    /// Series title.
    pub(crate) title: String,

    /// URL of the series page or endpoint.
    pub(crate) url: Url,

    /// Pagination of the chapters list.
    pub(crate) pagination: Pagination,
}

/// A chapter.
pub struct Chapter<'a> {
    /// Chapter ID.
    pub(crate) id: f64,

    /// Series containing this chapter.
    pub(crate) series: &'a Series,

    /// Volume name.
    pub(crate) volume: Option<String>,

    /// URL of the chapter page or endpoint.
    pub(crate) url: Url,
}

impl PartialOrd for Chapter<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialEq for Chapter<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A page.
pub struct Page<'a> {
    /// Chapter containing this page.
    pub(crate) chapter: &'a Chapter<'a>,

    /// URL of the page.
    pub(crate) main: Url,

    /// Fallback URL (used if the first one doesn't work), if any.
    pub(crate) fallback: Option<Url>,
}

/// Information about the pagination scheme used for the series.
pub(crate) struct Pagination {
    /// Total number of chapter available.
    pub(crate) chapter_count: u16,

    /// Number of chapters per page.
    pub(crate) page_size: u16,
}

impl Pagination {
    pub(crate) const fn new(chapter_count: u16, page_size: u16) -> Self {
        Self {
            chapter_count,
            page_size,
        }
    }

    /// Return on which page of the chapters list is a given chapter.
    pub(crate) fn get_page(&self, chapter: u16) -> u16 {
        // Clamp the chapter ID.
        let chapter = cmp::max(cmp::min(chapter, self.chapter_count), 1) - 1;

        // Ceiling division.
        ((self.chapter_count - chapter) + self.page_size - 1) / self.page_size
    }
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_size_10() {
        let pagination = Pagination::new(83, 10);

        assert_eq!(pagination.get_page(83), 1);
        assert_eq!(pagination.get_page(74), 1);

        assert_eq!(pagination.get_page(73), 2);

        assert_eq!(pagination.get_page(4), 8);

        assert_eq!(pagination.get_page(3), 9);
        assert_eq!(pagination.get_page(1), 9);
    }

    #[test]
    fn test_pagination_size_9() {
        let pagination = Pagination::new(485, 9);

        assert_eq!(pagination.get_page(485), 1);
        assert_eq!(pagination.get_page(477), 1);

        assert_eq!(pagination.get_page(476), 2);

        assert_eq!(pagination.get_page(9), 53);

        assert_eq!(pagination.get_page(8), 54);
        assert_eq!(pagination.get_page(1), 54);
    }

    #[test]
    fn test_pagination_size_6() {
        let pagination = Pagination::new(6, 6);
        for chapter in 1..=6 {
            assert_eq!(pagination.get_page(chapter), 1);
        }
    }
}

// }}}
