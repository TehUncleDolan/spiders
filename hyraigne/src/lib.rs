//! Hyraigne provides web spiders (a.k.a. web crawlers) to scrape websites like
//! `webtoons.com` or `mangadex.org` and helps you download chapters from there.

// Lints {{{

#![deny(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    rustdoc,
    missing_crate_level_docs,
    missing_docs,
    unreachable_pub,
    unsafe_code,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    warnings,
    clippy::all,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::panic,
    clippy::pattern_type_mismatch,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::verbose_file_reads,
    clippy::wrong_pub_self_convention,
    clippy::dbg_macro,
    clippy::expect_used,
    clippy::let_underscore_must_use,
    clippy::print_stdout,
    clippy::todo,
    clippy::unwrap_used,
    clippy::use_debug
)]
#![allow(
    // The 90’s called and wanted their charset back :p
    clippy::non_ascii_literal,
    // For Kuchiki imports.
    clippy::wildcard_imports,
    // It's easily outdated and doesn't bring that much value.
    clippy::missing_errors_doc,
)]

// }}}

mod error;
mod fs;
mod sites;
mod spiders;
mod types;
mod utils;

pub use error::{
    Error,
    Result,
};

// Can't merge the two "pub use" here.
// See https://github.com/rust-lang/rust/issues/64762
pub use sites::get_spider_for;
pub use sites::Site;

// Ditto, same bug…
pub use types::Chapter;
pub use types::Filter;
pub use types::Options;
pub use types::Page;
pub use types::Series;

// To test examples in the README.
// From https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
