use super::{
    chapter,
    models::{
        self,
        ChapterDetail,
        Response,
        SeriesWithChapter,
    },
    page,
    series,
    API_BASE_URL,
};
use crate::{
    spiders::HttpClient,
    Chapter,
    Error,
    Filter,
    Options,
    Page,
    Result,
    Series,
};
use once_cell::unsync::Lazy;
use regex::Regex;
use std::path::PathBuf;
use url::Url;

/// A web spider for `https://mangadex.org`.
pub(crate) struct Site {
    spider: HttpClient,
    output: PathBuf,
}

impl Site {
    /// Initialize the web spider with the given options.
    pub(crate) fn new(options: Options) -> Self {
        Self {
            spider: HttpClient::new(options.delay, options.retry, None),
            output: options.output,
        }
    }
}

impl crate::Site for Site {
    fn get_series(&self, url: &Url) -> Result<Series> {
        let endpoint = endpoint_from_url(&url)?;

        log::info!("scraping series info from {}…", endpoint.as_str());

        let response: Response<models::Series> =
            self.spider.get_json(&endpoint)?;
        let series =
            series::extract_from_response(response).map_err(|err| {
                Error::Scraping(format!(
                    "failed to scrape serie from {}: {}",
                    endpoint.as_str(),
                    err
                ))
            })?;

        Ok(series)
    }

    fn get_chapters<'a>(
        &self,
        series: &'a Series,
        filter: Filter,
    ) -> Result<Vec<Chapter<'a>>> {
        log::info!("scraping chapter links for series {}…", series.title);

        let mut url = series.url.clone();
        url.set_query(Some("include=chapters"));

        let response: Response<SeriesWithChapter> =
            self.spider.get_json(&url)?;
        let chapters =
            chapter::extract_from_response(response, &series, &filter)
                .map_err(|err| {
                    Error::Scraping(format!(
                        "failed to scrape chapters from {}: {}",
                        url.as_str(),
                        err
                    ))
                })?;
        log::debug!("found {} chapters", chapters.len());

        let start = f64::from(*filter.range.start());
        let end = f64::from(*filter.range.end());
        let range = start..=end;
        // Trim the chapters list to keep only the requested chapters.
        let mut chapters = chapters
            .into_iter()
            // Chapter IDs are always positive and under u16::MAX.
            .filter(|chapter| range.contains(&chapter.id))
            .collect::<Vec<_>>();

        #[allow(clippy::expect_used)] // Shouldn't have NaN & friends here…
        chapters
            .sort_by(|a, b| a.partial_cmp(b).expect("abnormal float as ID"));
        log::debug!("selected {} chapters", chapters.len());

        Ok(chapters)
    }

    fn get_pages<'a>(&self, chapter: &'a Chapter<'_>) -> Result<Vec<Page<'a>>> {
        log::info!("scraping page links for chapter {}…", chapter.id);

        let response: Response<ChapterDetail> =
            self.spider.get_json(&chapter.url)?;
        let pages =
            page::extract_from_response(&response, chapter).map_err(|err| {
                Error::Scraping(format!(
                    "failed to pages from {}: {}",
                    chapter.url.as_str(),
                    err
                ))
            })?;

        log::debug!("found {} pages in chapter {}", pages.len(), chapter.id);

        Ok(pages)
    }

    fn mkdir(&self, chapters: &[Chapter<'_>]) -> Result<()> {
        for chapter in chapters {
            let path = chapter.path(&self.output);
            crate::fs::mkdir_p(&path)?;
        }

        Ok(())
    }

    fn download(&self, pages: &[Page<'_>]) -> Result<()> {
        log::info!(
            "downloading {} pages for chapter {}…",
            pages.len(),
            pages[0].chapter.id,
        );

        let mut bytes: Vec<u8> = Vec::new();

        for page in pages.iter() {
            // Compute the image path.
            let path = page.path(&self.output);

            // Skip it if it has already been downloaded.
            if path.exists() {
                log::debug!("{} already exists, skip", path.display());
                continue;
            }

            log::info!("downloading {}…", path.display());
            self.spider
                .get_image(&page.main, &page.chapter.url, &mut bytes)
                .or_else(|err| {
                    page.fallback.as_ref().map_or(Err(err), |fallback_url| {
                        self.spider.get_image(
                            fallback_url,
                            &page.chapter.url,
                            &mut bytes,
                        )
                    })
                })?;

            crate::fs::atomic_save(&path, &bytes)?;

            bytes.clear();
        }

        Ok(())
    }
}

// Convert a series URL into the corresponding API endpoint.
#[allow(clippy::expect_used)] // Regexp is hardcoded and valid.
fn endpoint_from_url(url: &Url) -> Result<Url> {
    let extract_id = Lazy::new(|| {
        Regex::new(r#"^/title/(?P<id>\d+)"#).expect("invalid series ID regexp")
    });

    let id = extract_id
        .captures(url.path())
        .ok_or_else(|| {
            Error::Scraping(format!("series ID not found in {}", url.as_str()))
        })?
        .name("id")
        .expect("invalid capture group for series ID");

    let endpoint = format!("{}/manga/{}", API_BASE_URL, id.as_str());

    Url::parse(&endpoint).map_err(|err| {
        Error::Scraping(format!(
            "invalid series endpoint {}: {}",
            endpoint, err
        ))
    })
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_from_url() {
        let url =
            Url::parse("https://mangadex.org/title/642/kingdom/").unwrap();

        let endpoint = endpoint_from_url(&url).unwrap();

        assert_eq!(endpoint.as_str(), "https://api.mangadex.org/v2/manga/642");
    }
}

// }}}
