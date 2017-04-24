# Helix Release Process

1. Verify that CI is passing
1. Update all version number references. (There are a lot, do a search.)
1. Update CHANGELOG.md
1. Run `./scripts/test-release` to make sure packages are ready for release.
1. Commit and push version bump and changelog.
1. Tag version in Git, prefixed with 'v' and push with `git push --tags`

### In `./ruby`

1. `gem build helix-runtime.gemspec`
1. `gem push helix-runtime-VERSION.gem`

### In `./crates/libcruby-sys`

1. `HELIX_LIB_DIR=$PWD cargo publish`

### In `.`

1. `cargo publish`
