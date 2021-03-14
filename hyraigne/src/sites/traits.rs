use crate::{
    Chapter,
    Filter,
    Options,
    Page,
    Result,
    Series,
};
use url::Url;

/// A website scraper.
pub trait Site {
    /// Build a site-specific web spider with the given options.
    fn new_from_options(options: Options) -> Self;

    /// Fetch the series at `url`.
    fn get_series(&self, url: &Url) -> Result<Series>;

    /// Fetch the chapters of `series` and filter them as specified.
    fn get_chapters<'a>(
        &self,
        series: &'a Series,
        filter: Filter,
    ) -> Result<Vec<Chapter<'a>>>;

    /// Fetch the pages of the given chapter.
    fn get_pages<'a>(&self, chapter: &'a Chapter<'_>) -> Result<Vec<Page<'a>>>;

    /// Create the required directory hierarchy to download the pages.
    fn mkdir(&self, chapters: &[Chapter<'_>]) -> Result<()>;

    /// Download the given pages.
    fn download(&self, pages: &[Page<'_>]) -> Result<()>;
}
