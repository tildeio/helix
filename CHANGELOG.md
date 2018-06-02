## 0.7.4 (June 2, 2018)

* [BUGFIX] Ensure classes with a `struct` also defines `initialize`
* [BUGFIX] Ensure Ruby Strings have the correct encoding before performing coercion
* [BUGFIX] Abort rake task when `cargo build` fails
* [IMPROVEMENT] Add coercion for tuples

## 0.7.3 (March 6, 2018)

* [IMPROVEMENT] Various improvements to the underlying `libcruby-sys` library

## 0.7.2 (October 9, 2017)

* [IMPROVEMENT] Add coercion for Symbols
* [IMPROVEMENT] Allow consuming self in methods

## 0.7.1 (October 6, 2017)

* [IMPROVEMENT] Add coercion for `Vec` and `HashMap`

## 0.7.0 (October 3, 2017)

* [IMPROVEMENT] Add error message for parse errors
* [IMPREVEMENT] Support #[ruby_name] remapping for classes
* [IMPREVEMENT] Support returning an exception to Ruby (via `Result` coercion)
* [BUGFIX] Fix build erros for project names with more than one `-`
* [EXPERIMENTAL] Make it possible to implement `FromRuby` and `ToRuby` for custom types

## 0.6.4 (September 7, 2017)

* [BUGFIX] Compile 32-bit windows .lib with 32-bit toolchain

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
