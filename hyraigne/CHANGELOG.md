# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Possible sections are:

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.1.2] - 2021-03-19

### Added

- Web spider for https://mangadex.org.
- Chapter can be filtered by language.
- A list of preferred scantrad team can be supplied, it will be used to select a
  chapter when more than one version are available.

## [0.1.1] - 2021-03-18

### Added

- Web spider for https://webtoonscan.com.
- Add a factory method to instanciate the right spider according to the URL.

### Changed

- Log messages for page downloading are now at info level instead of debug.

## [0.1.0] - 2021-03-18

### Added

- Web spider for https://webtoons.com.
