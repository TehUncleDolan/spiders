# Hyraigne

[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

Hyraigne is a library that provides web spiders (a.k.a. web crawlers) to scrape
websites like `webtoons.com` or `mangadex.org` and helps you download chapters
from there.

## Usage

Here's a simple example that download a series from `webtoons.com`:

```rust,no_run
use url::Url;

fn main() {
    let url = Url::parse("https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95")
        .expect("invalid URL");
    let opts = hyraigne::Options::new(1000, 3, "/home/me/Webtoons".into());
    let filter = hyraigne::Filter::new(0..=u16::MAX, None, Vec::new());
    let spider = hyraigne::get_spider_for(&url, opts).expect("unsupported URL");

    let series = spider.get_series(&url)
        .expect("failed to scrape series info");
    let chapters = spider.get_chapters(&series, filter)
        .expect("failed to scrape chapter list");

    spider.mkdir(&chapters).expect("failed to setup workdir");
    for chapter in chapters {
        let pages = spider.get_pages(&chapter)
            .expect("failed to scrape page list");
        spider.download(&pages)
            .expect("failed to download pages");
    }
}
```

## Supported websites

- [MangaDex](https://mangadex.org/)
- [MangaKakalot](https://mangakakalot.com/)
- [WEBTOON](https://www.webtoons.com/)
- [WebtoonScan](https://webtoonscan.com/)

## About the name

“Hyraigne” is an old word, from Middle French, for “spider”.
