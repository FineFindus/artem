# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2022-03-01

### Added

- Changed version to 0.4.1, since github actions would not use the right files otherwise
- Fixed error with tar command in cd

## [0.4.0] - 2022-03-01

### Added

- README now contains an installation section
- Use the `--background` flag to let the ascii chars have a background color. Might be useful for images with dark backgrounds.
- Use the `--invert` flag to change the brightness of the used characters. Can be useful for images with a dark background
- README now lists some example formats that can be used
- Tab completions now works in other shells as well (fish and zsh in deb package)
- Removed linting problems found by clippy
- CI tests now against the stable, beta and nightly rust version
- CI now checks for clippy warnings
- Changelog file to document changes to the project
- A Feature template can be used to easily request features over Github

### Changed

- Logging no longer logs the date, since it is not needed
- Man Page String are now formatted correctly

## [0.3.0] - 2022-02-25

### Added

- Logging with different verbosity levels to help debugging
- `verbose` flag can be used to change the verbosity, defaults to `error`
