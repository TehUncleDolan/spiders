use super::{
    models::{
        self,
        Response,
        SeriesWithChapter,
    },
    series,
    API_BASE_URL,
};
use crate::{
    Chapter,
    Error,
    Filter,
    Result,
    Series,
};
use once_cell::unsync::Lazy;
use std::{
    collections::{
        btree_map::Entry,
        BTreeMap,
        HashMap,
    },
    path::{
        Path,
        PathBuf,
    },
};
use url::Url;

/// Extract every chapter from the API response.
pub(super) fn extract_from_response<'a>(
    response: Response<SeriesWithChapter>,
    series: &'a Series,
    filter: &Filter,
) -> Result<Vec<Chapter<'a>>> {
    // Build a mapping to get the goup name from the group ID.
    let group_index = response
        .data
        .groups
        .into_iter()
        .map(|group| (group.id, group.name))
        .collect::<HashMap<_, _>>();

    // First, filter by language.
    let chapters = response
        .data
        .chapters
        .into_iter()
        .filter(|chapter| chapter.language == filter.language);

    // Then, filter out duplicate (same chapter translated by several teams).
    let chapters =
        dedup_chapters(chapters, &filter.preferred_groups, &group_index);

    // Finally, build the chapter objetcs.
    chapters
        .into_iter()
        .map(|(_, chapter)| {
            let id = chapter.chapter.parse().map_err(|err| {
                Error::Scraping(format!(
                    "invalid chapter ID {}: {}",
                    chapter.chapter, err
                ))
            })?;
            let endpoint = format!("{}/chapter/{}", API_BASE_URL, chapter.id);
            let volume = (!chapter.volume.is_empty()).then(|| chapter.volume);

            Ok(Chapter {
                id,
                series,
                volume,
                url: Url::parse(&endpoint).map_err(|err| {
                    Error::Scraping(format!(
                        "invalid chapter endpoint {}: {}",
                        endpoint, err
                    ))
                })?,
            })
        })
        .collect()
}

/// Get a path to the directory where where the chapter will be saved.
pub(super) fn get_path(path: &Path, chapter: &Chapter<'_>) -> PathBuf {
    let unknown_volume = Lazy::new(|| "XX".to_owned());
    let dirname = crate::fs::sanitize_name(&format!(
        "{} {:0>2}",
        chapter.series.title,
        &chapter.volume.as_ref().unwrap_or(&unknown_volume)
    ));
    let path = series::get_path(path, chapter.series);

    [path, dirname].iter().collect()
}

/// Filter our duplicated chapters based on a computed score.
fn dedup_chapters(
    chapters: impl Iterator<Item = models::Chapter>,
    preferred_groups: &[String],
    group_index: &HashMap<u32, String>,
) -> BTreeMap<String, models::Chapter> {
    let mut result = BTreeMap::new();

    for chapter in chapters {
        match result.entry(chapter.chapter.clone()) {
            Entry::Vacant(slot) => {
                slot.insert(chapter);
            },
            // Choose between two versions of the same chapter.
            Entry::Occupied(mut slot) => {
                let current = slot.get();
                let current_rank =
                    get_score(current, preferred_groups, group_index);
                let new_rank =
                    get_score(&chapter, preferred_groups, group_index);

                // Take the best score, or the most recent if equals.
                if (new_rank < current_rank)
                    || (new_rank == current_rank
                        && chapter.timestamp > current.timestamp)
                {
                    slot.insert(chapter);
                }
            },
        };
    }

    result
}

/// Compute a score for the chapter.
///
/// The score is based on the team that scanlated the chapter: teams that are in
/// the list of preferred groups get a better score.
///
/// Lower is better.
fn get_score(
    chapter: &models::Chapter,
    preferred_groups: &[String],
    group_index: &HashMap<u32, String>,
) -> usize {
    let default = usize::MAX;

    chapter
        .groups
        .iter()
        .map(|group_id| {
            match group_index.get(group_id) {
                Some(name) => {
                    preferred_groups
                        .iter()
                        .position(|group| group == name)
                        .unwrap_or(default)
                },
                None => default,
            }
        })
        .min()
        .unwrap_or(default)
}

// Tests {{{

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        types::Pagination,
        Filter,
    };

    #[test]
    fn test_scraping() {
        let series = Series {
            title: "Example".to_owned(),
            url: Url::parse("http://example.com/").unwrap(),
            pagination: Pagination::new(0, 0),
        };
        let language = "gb".to_owned();
        let filter = Filter::new(0..=u16::MAX, Some(language), Vec::new());
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("testdata/mangadex.org/series_details.json");
        let json = std::fs::read_to_string(&path).expect("test data");
        let response = serde_json::from_str(&json).expect("invalid JSON");

        let chapters =
            extract_from_response(response, &series, &filter).unwrap();

        assert_eq!(chapters.len(), 673);
    }
}

// }}}
