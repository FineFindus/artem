# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.2] - 2022-05-28

### Changed

- Fixed issues with release not recognizing license

## [1.1.1] - 2022-05-28

### Changed

- Fixed cross compilation, by switch from `https-native` to `https-bundled-probe`, since dynamically linked openssl is not supported by cross

## [1.1.0] - 2022-05-28

### Added

- It is now possible to input multiple files
- Improved handling of `.ansi`/`.ans` files ( includes more warnings)
- Improved documentation of output files
- Added `web_image` feature, which allows URL instead of path input. It is enabled by default
- Authors and description will now be shown in the command help text and man page
- New image has been added for testing purposes
- The `--centerX` and `--centerY` can be used to center the image in the terminal
- `artem` is now available as an aur package (artem) as well as as a aur binary package (artem-bin)

### Changed

- Added `--release` flag to example build command (Thanks to @talwat)
- Fixed failing doc test on rust nightly
- Fixed an error that could occur when the image was only 1x1
- Fixed an issue where the border was not applied correctly if the image was only a single pixel wide
- Adapted tests to work with multiple file inputs
- Increased minimum rust version in CI to 1.60.0
- The clap command now uses information from Cargo.toml, including version, authors, description and name
- Improved command help texts
- Rewrote description of the command help
- Rewrote description of the `.deb` package
- Renamed `--character` input text to be **_characters_** instead of **_density_**
- Argument tests have been refactored to their own files
- Test now complete much faster (down to 5s from 104s) due to using a smaller image, since the large one was unnecessary used

## [1.0.3] - 2022-05-05

### Changed

- Fixed error in windows releases building process
- Fixed error with brew tap

## [1.0.2] - 2022-05-04

### Changed

- Fixed error in windows releases

## [1.0.1] - 2022-05-04

### Changed

- Fixed error in release workflow

## [1.0.0] - 2022-05-04

### Added

- A [homebrew tap](https://github.com/FineFindus/homebrew-tap) is now available
- Cargo publishing is now done in the release workflow
- The release workflow now updates the homebrew tap
- Shell completion files and the man page are now contained in the compressed released files
- A new README sections explains how to install the completion files
- The new `--outline` flag will only produce an only of the image
- When using the `--hysteresis` flag along the `--outline` flag the ascii art will be even more outlined, with less imperfections
- Added more test cases and examples to the README to cover the newly added functionality
- Major refactoring of the code
- Artem is now a library, which is used by the command-line interface
- Due to a refactoring of the code, the output ascii image now resembles the input image more closely

### Changed

- Overhauled the installation section in the README, it now contains much more detailed installations instructions
- Switched from `f64` to `f32`, since the additional precision has literally **_no_** impact (it gets rounded/cut away), but yields worse performance
- Refactored `average_color` to be iterator based, according to some microbenchmarks, this makes it a few nanoseconds faster
- Refactored the `convert`, `blur`, `apply_sober` and `edge_tracking` functions to use `iterator`s instead of for loops. This removes a lot of nasty and hart to deal with bug, especially together with multi-threading
- Removed multithreading, it was a constant source of bugs, since pixels can't be split for threads/output characters. It also rarely brought noticeable performance improvements
- The new iterator-based implementation opens the possibility to use [rayon](https://crates.io/crates/rayon) in the future
- Fixed a crash which could occur when piping to a file using the maximum terminal size
- Fixed a bug, where the `--height` argument would not actually use the correct height and being a bit too high

## [0.6.1] - 2022-03-24

### Added

- Linux Binaries will now also be compiled with `musl`
- Completion scripts for the `.deb` will be copied in the CD process
- With mscv compiled windows binaries are available as an alternative to the gnu compiled ones
- MacOS binaries for (x86 and arm) have been added to the CD process

## [0.6.0] - 2022-03-24

### Added

- When using an html output file, artem will now converted the result to html, this also works with .ans files respectively
- More Documentation to better describe the code
- The `--border` flag can be used to create a border around the ascii image
- The `--flipX` flag can be used horizontally flip the image
- The `--flipY` flag can be used vertically flip the image
- Two more tests, which fully compare the results

### Changed

- Major refactoring

## [0.5.1] - 2022-03-14

### Changed

- Using a new workflow job for the windows build

## [0.5.0] - 2022-03-14

### Added

- Release builds are now available for more targets (linux x64 and arm) and windows (using gnu-target)

### Changed

- Using the `--width` argument now correctly resizes the image
- Using the `--height` argument now uses the correct height of the terminal
- Using multiple Threads now display the full image instead of leaving a few pixels out
- Updated the example image in the README to reflect the changes

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
