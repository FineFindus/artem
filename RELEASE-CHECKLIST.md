# Release Checklist

These steps should be performed when making a new release. Do not commit marked checks in this file.

## Pre-release Test Checklist

- [x] Local and Remote branches are synced
- [x] All the tests are passing
- [ ] Continuous Integration is passing
- [x] README has been updated
- [x] All changes were documented in the Changelog
- [x] Added the correct semantic version in the Changelog
- [x] Changed the changes from Unreleased to the new version in the Changelog
- [x] Updated the version number in Cargo.toml
- [x] Updated the version number in in cli.rs
- [x] Example images still represents the project accurately
- [x] Example commands still represents the project accurately
- [ ] Copied the changes to a new release
- [ ] Build artifacts have been attached to the release through continuous delivery
- [x] Cargo deb builds the correct package
- [x] Documentation has been updated to reflect the changes
- [x] Manpage contains the correct help
- [x] Tab-completions works in a all supported shells

## Post-release Test Checklist

- [ ] Installation instructions work using the released artefact
