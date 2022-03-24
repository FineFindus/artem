# Release Checklist

These steps should be performed when making a new release. Do not commit marked checks in this file.

## Pre-release Test Checklist

- [ ] Local and Remote branches are synced
- [ ] All the tests are passing
- [ ] Continuous Integration is passing
- [ ] README has been updated
- [ ] All changes were documented in the Changelog
- [ ] Added the correct semantic version in the Changelog
- [ ] Changed the changes from Unreleased to the new version in the Changelog
- [ ] Updated the version number in Cargo.toml
- [ ] Updated the version number in in cli.rs
- [ ] Example images still represents the project accurately
- [ ] Example commands still represents the project accurately
- [ ] Copied completions file from the OUT_DIR to deployment/assets
- [ ] Copied the changes to a new release
- [ ] Build artifacts have been attached to the release through continuous delivery
- [ ] Cargo deb builds the correct package
- [ ] Documentation has been updated to reflect the changes
- [ ] Manpage contains the correct help
- [ ] Tab-completions works in a all supported shells

## Post-release Test Checklist

- [ ] Installation instructions work using the released artefact
