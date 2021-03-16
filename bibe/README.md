# Bibe

[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

Bibe is a command-line tool that allows you to download every chapter (or a
subset) of a series from websites like `webtoons.com` or `mangadex.org`.

## Usage

```bash
USAGE:
    bibe [OPTIONS] --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --begin <begin>      Start downloading from this chapter [env: BIBE_BEGIN=]
    -d, --delay <delay>      Delay between each request (in ms) [env: BIBE_DELAY=] [default: 1000]
    -e, --end <end>          Stop downloading after this chapter [env: BIBE_END=]
    -o, --output <output>    Output directory [env: BIBE_OUTPUT=] [default: .]
    -r, --retry <retry>      Max number of retry for HTTP requests [env: BIBE_RETRY=] [default: 3]
    -u, --url <url>          Series URL [env: BIBE_URL=]
```

The simplest invocation only requires you to specify the URL of the series you
want to download, the other options have sensible defaults.

```bash
bibe -u 'https://www.webtoons.com/fr/thriller/hell-is-other-people/list?title_no=1841'
```

If you only want to download a subset of the chapters, you can specify a range.
For example, the following command will download the first 10 chapters under the
specified directory:

```bash
bibe --url 'https://www.webtoons.com/fr/thriller/hell-is-other-people/list?title_no=1841' \
     --begin 1
     --end 10
     --output ~/Documents/Books/Webtoons
```
