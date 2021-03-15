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
