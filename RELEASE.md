# Helix Release Process

0. Verify that CI is passing
0. Update all version number references. (There are a lot, do a search.)
0. Update CHANGELOG.md
0. Run `./scripts/test-release` to make sure packages are ready for release.
0. Commit and push version bump and changelog.
0. Tag version in Git, prefixed with 'v' and push with `git push --tags`

### In `./ruby`

0. `gem build helix-runtime.gemspec`
0. `gem push helix-runtime-VERSION.gem`

### In `./crates/libcruby-sys`

0. `HELIX_LIB_DIR=$PWD cargo publish`

### In `.`

0. `cargo publish`
