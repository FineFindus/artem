# Release Checklist

These steps should be performed when making a new release. Do not commit marked checks in this file.

## Pre-release Test Checklist

### Before Committing
- [ ] Local and Remote branches are synced
- [ ] All the tests are passing
- [ ] Continuous Integration is passing
- [ ] `cargo clippy` finds no errors 
- [ ] `cargo publish --dry-run` passes 
- [ ] README has been updated
- [ ] All changes were documented in the Changelog
- [ ] Added the correct semantic version in the Changelog
- [ ] Changed the changes from Unreleased to the new version in the Changelog
- [ ] Updated the version number in Cargo.toml
- [ ] Build version matches version in Cargo.toml
- [ ] Example images still represents the project accurately
- [ ] Example commands still represents the project accurately
- [ ] Documentation has been updated to reflect the changes
- [ ] Manpage contains the correct help
- [ ] Tab-completions works in a all supported shells
### After Committing
- [ ] Copied the changes to a new release
- [ ] Build artifacts have been attached to the release through continuous delivery
- [ ] Cargo deb builds the correct package
## Post-release Test Checklist

- [ ] Installation instructions work using the released artefact
