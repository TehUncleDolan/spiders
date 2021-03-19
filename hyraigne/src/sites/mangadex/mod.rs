mod chapter;
mod models;
mod page;
mod series;
mod site;

/// Mangadex API address.
pub(super) const API_BASE_URL: &str = "https://api.mangadex.org/v2";

pub(crate) use site::Site;
