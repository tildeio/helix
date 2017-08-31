## 0.6.3 (August 30, 2017)

* [BUGFIX] Include *.lib in libcruby-sys

## 0.6.2 (August 29, 2017)

* [DEPRECATION] Deprecate passing project name to `BuildTask`

## 0.6.1 (May 18, 2017)

* [BUGFIX] Fix path in copy_dll task for Windows installation
* [BUGFIX] Fix compatibility with serde_json
* [BUGFIX] Correctly handle projects with dashes in their name
* [IMPROVEMENT] Better language around coercion failures

## 0.6.0 (May 3, 2017)

* [BUGFIX] Raise TypeError on bad initialize
* [BUGFIX] Fix Windows release
* [IMPROVEMENT] Fewer allocations by making function signatures take `&CStr`s.
* [IMPROVEMENT] Improve installation process when running examples
* [IMPROVEMENT] Better handling of unbuilt helix_runtime/native during development
* [IMPROVEMENT] Fix var shadowing and ambiguous args in ruby warnings
* [IMPROVEMENT] Bump cslice crate: 0.2 -> 0.3.

## 0.5.0 (April 26, 2017)

* First official release
