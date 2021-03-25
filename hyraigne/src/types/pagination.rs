use std::cmp;

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
