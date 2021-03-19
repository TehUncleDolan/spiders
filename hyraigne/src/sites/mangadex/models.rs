//! Types exposed by the Mangadex API.
//!
//! This doesn't cover the entire Mangadex API, only the subset needed to
//! download chapters.

use serde::Deserialize;
use url::Url;

/// Response fron the Mangadex API.
#[derive(Debug, Deserialize)]
pub(super) struct Response<T> {
    pub(super) code: u16,
    pub(super) status: String,
    pub(super) data: T,
}

/// Series info with chapters included.
#[derive(Debug, Deserialize)]
pub(super) struct SeriesWithChapter {
    pub(super) manga: Series,
    pub(super) chapters: Vec<Chapter>,
    pub(super) groups: Vec<Group>,
}

/// Series info.
#[derive(Debug, Deserialize)]
pub(super) struct Series {
    pub(super) id: u64,
    pub(super) title: String,
}

/// Chapter info.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Chapter {
    pub(super) id: u64,
    pub(super) manga_title: String,
    pub(super) volume: String,
    pub(super) chapter: String,
    pub(super) language: String,
    pub(super) groups: Vec<u32>,
    pub(super) timestamp: u64,
}

/// Group info
#[derive(Debug, Deserialize)]
pub(super) struct Group {
    pub(super) id: u32,
    pub(super) name: String,
}

/// Chapter detailed info.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ChapterDetail {
    pub(super) id: u64,
    pub(super) hash: String,
    pub(super) volume: String,
    pub(super) chapter: String,
    pub(super) language: String,
    pub(super) pages: Vec<String>,
    pub(super) server: Url,
    pub(super) server_fallback: Url,
}
