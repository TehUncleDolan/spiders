use super::Pagination;
use std::path::{
    Path,
    PathBuf,
};
use url::Url;

/// A series.
pub struct Series {
    /// Series title.
    pub(crate) title: String,

    /// URL of the series page or endpoint.
    pub(crate) url: Url,

    /// Pagination of the chapters list.
    pub(crate) pagination: Pagination,
}

impl Series {
    /// Get a path to the directory where where the series will be saved.
    pub(super) fn path(&self, basedir: &Path) -> PathBuf {
        let dirname = crate::fs::sanitize_name(&self.title);

        [basedir, &dirname].iter().collect()
    }
}
