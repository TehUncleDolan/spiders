/// Format and correctly pad the chapter ID.
pub(crate) fn format_chapter_id(id: f64) -> String {
    let fract = id.fract();
    let width = if fract == 0.0 {
        3
    } else {
        2 + format!("{}", fract).len()
    };
    format!("{:0width$}", id, width = width)
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_chapter_id() {
        assert_eq!(format_chapter_id(3.0), "003");
        assert_eq!(format_chapter_id(3.5), "003.5");
        assert_eq!(format_chapter_id(30.5), "030.5");
        assert_eq!(format_chapter_id(300.5), "300.5");
    }
}

// }}}
