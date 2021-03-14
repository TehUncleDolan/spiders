//! Provides various web spiders to scrape a wide range of websites, from simple
//! to complex ones (relying on JS fuckery).

mod http;

pub(crate) use http::Spider as HttpClient;
