use std::ops::RangeInclusive;

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
