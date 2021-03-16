//! Bibe is a command-line tool that allows you to download every chapter (or a
//! subset) of a series from websites like `webtoons.com` or `mangadex.org`.

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

use anyhow::{
    ensure,
    Result,
};
use clap::Clap;
use env_logger::Env;
use hyraigne::Site;
use std::path::PathBuf;
use url::Url;

/// Man{ga,hua,hwa} downloader, can download entire series (default) or a subset
/// of chapter only.
#[derive(Clap)]
#[clap(version, author)]
struct Args {
    /// Series URL.
    #[clap(short, long, env = "BIBE_URL", parse(try_from_str = Url::parse))]
    url: Url,

    /// Delay between each request (in ms).
    #[clap(short, long, env = "BIBE_DELAY", default_value = "1000")]
    delay: u16,

    /// Max number of retry for HTTP requests.
    #[clap(short, long, env = "BIBE_RETRY", default_value = "3")]
    retry: u8,

    /// Output directory.
    #[clap(
        short,
        long,
        env = "BIBE_OUTPUT",
        parse(from_os_str),
        default_value = "."
    )]
    output: PathBuf,

    /// Start downloading from this chapter.
    #[clap(short, long, env = "BIBE_BEGIN")]
    begin: Option<u16>,

    /// Stop downloading after this chapter.
    #[clap(short, long, env = "BIBE_END")]
    end: Option<u16>,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(
        Env::default().default_filter_or("hyraigne=info"),
    )
    .init();

    let args: Args = Args::parse();

    let begin = args.begin.unwrap_or(u16::MIN);
    let end = args.end.unwrap_or(u16::MAX);
    ensure!(begin <= end, "`begin` must be lower than `end`");
    let range = begin..=end;

    let opts = hyraigne::Options::new(args.delay, args.retry, args.output);
    let filter = hyraigne::Filter::new(range);
    let spider = hyraigne::Webtoons::new_from_options(opts);

    let series = spider.get_series(&args.url)?;
    let chapters = spider.get_chapters(&series, filter)?;

    spider.mkdir(&chapters)?;
    for chapter in chapters {
        let pages = spider.get_pages(&chapter)?;
        spider.download(&pages)?;
    }

    Ok(())
}
