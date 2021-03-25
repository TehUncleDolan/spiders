use std::{
    cmp,
    path::PathBuf,
    time,
};

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
