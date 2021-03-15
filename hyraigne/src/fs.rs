//! Filesystem helpers.

use crate::{
    Error,
    Result,
};
use once_cell::unsync::Lazy;
use regex::Regex;
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};
use url::Url;

/// Clean a name to safely use it as directory name.
#[allow(clippy::expect_used)] // Regexp are hardcoded and correct.
pub(crate) fn sanitize_name(name: &str) -> PathBuf {
    // Linux only is not that restrictive, but Windows is another story...
    // See https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file
    let dir_illegal_chars = Lazy::new(|| {
        Regex::new(r#"[/\?<>\\:\*\|"]"#).expect("invalid chars regexp")
    });
    let dir_illegal_trailing =
        Lazy::new(|| Regex::new(r#"[\. ]+$"#).expect("invalid trailing regex"));

    let name = dir_illegal_trailing.replace(name, "");
    dir_illegal_chars
        .replace_all(&name, "_")
        .into_owned()
        .into()
}

/// Recursively create a directory and all of its parent if necessary.
pub(crate) fn mkdir_p(path: &Path) -> Result<()> {
    fs::create_dir_all(&path).map_err(|err| {
        Error::Filesystem {
            operation: "mkdir",
            target: path.to_path_buf(),
            source: err,
        }
    })
}

// Extract the file extension from the image URL.
#[allow(clippy::expect_used)] // Highly unlikely to have bad UTF-8 in file ext.
pub(crate) fn extname_from_url(url: &Url) -> &str {
    Path::new(url.path())
        .extension()
        .map_or("jpg", |ext| ext.to_str().expect("invalid extension"))
}

/// Write a file atomically.
pub(crate) fn atomic_save(path: &Path, data: &[u8]) -> Result<()> {
    let mut tmp_path = path.to_path_buf();
    tmp_path.set_extension("tmp");

    fs::write(&tmp_path, data).map_err(|err| {
        Error::Filesystem {
            operation: "write",
            target: tmp_path.clone(),
            source: err,
        }
    })?;

    fs::rename(&tmp_path, path).map_err(|err| {
        Error::Filesystem {
            operation: "rename",
            target: path.to_path_buf(),
            source: err,
        }
    })
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_trailing() {
        assert_eq!(sanitize("foo   "), "foo");
        assert_eq!(sanitize("foo."), "foo");
        assert_eq!(sanitize("foo. ."), "foo");
        assert_eq!(sanitize("foo. . "), "foo");
    }

    #[test]
    fn test_sanitize_invalid() {
        assert_eq!(sanitize("foo/bar/"), "foo_bar_");
        assert_eq!(sanitize("foo:bar"), "foo_bar");
        assert_eq!(sanitize("foo?bar"), "foo_bar");
        assert_eq!(sanitize("foo|bar"), "foo_bar");
        assert_eq!(sanitize("foo*bar"), "foo_bar");
        assert_eq!(sanitize("foo>bar"), "foo_bar");
        assert_eq!(sanitize("foo<bar"), "foo_bar");
        assert_eq!(sanitize("foo\\bar"), "foo_bar");
        assert_eq!(sanitize("foo\"bar"), "foo_bar");
    }
}

// }}}
