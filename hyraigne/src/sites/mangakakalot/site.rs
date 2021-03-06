use super::{
    chapter,
    page,
    series,
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
use std::path::PathBuf;
use url::Url;

/// A web spider for `https://mangakakalot.com`.
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
        log::info!("scraping series info from {}…", url.as_str());

        let html = self.spider.get_html(url)?;
        let series = series::scrape_from_html(&html).map_err(|err| {
            Error::Scraping(format!(
                "failed to scrape serie from {}: {}",
                url.as_str(),
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

        let html = self.spider.get_html(&series.url)?;
        let chapters =
            chapter::scrape_from_html(&html, &series).map_err(|err| {
                Error::Scraping(format!(
                    "failed to scrape chapters from {}: {}",
                    series.url.as_str(),
                    err
                ))
            })?;
        log::debug!("found {} chapters", chapters.len());

        // Trim the chapters list to keep only the requested chapters.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let mut chapters = chapters
            .into_iter()
            // Chapter IDs are always positive and under u16::MAX.
            .filter(|chapter| filter.range.contains(&(chapter.id as u16)))
            .collect::<Vec<_>>();

        #[allow(clippy::expect_used)] // Shouldn't have NaN & friends here…
        chapters
            .sort_by(|a, b| a.partial_cmp(b).expect("abnormal float as ID"));
        log::debug!("selected {} chapters", chapters.len());

        Ok(chapters)
    }

    fn get_pages<'a>(&self, chapter: &'a Chapter<'_>) -> Result<Vec<Page<'a>>> {
        log::info!("scraping page links for chapter {}…", chapter.id);

        let html = self.spider.get_html(&chapter.url)?;
        let pages = page::scrape_from_html(&html, chapter).map_err(|err| {
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
                .get_image(&page.main, &page.chapter.url, &mut bytes)?;

            crate::fs::atomic_save(&path, &bytes)?;

            bytes.clear();
        }

        Ok(())
    }
}
