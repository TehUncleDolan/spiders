use crate::utils;
use std::{
    cmp,
    path::{
        Path,
        PathBuf,
    },
};
use url::Url;

use super::Series;

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

impl Chapter<'_> {
    /// Get a path to the directory where where the chapter will be saved.
    pub(crate) fn path(&self, basedir: &Path) -> PathBuf {
        // If volume is known, chapter will be stored in the volume's directory.
        // Otherwise, chapter will be saved in its own directory.
        let dirname = if let Some(volume) = self.volume.as_ref() {
            format!("{} {:0>2}", self.series.title, volume)
        } else {
            let chapter_id = utils::format_chapter_id(self.id);
            format!("{} {:03}", self.series.title, chapter_id)
        };
        let dirname = crate::fs::sanitize_name(&dirname);
        let path = self.series.path(basedir);

        [path, dirname].iter().collect()
    }
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
