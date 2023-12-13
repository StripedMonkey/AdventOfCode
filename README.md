# Advent Of Code

Advent of Code is a yearly advent competition following the Advent calendar. This is a repository of my attempts for 2023, and perhaps previous years if I ever get around to it.

## Inputs

Note, no inputs, are stored within this project. Inputs are read from the `year-<year>/day-<day>/` directory.
Examples are included within to ensure tests will pass.

All rust tests can be run by:

```shell
cargo test
```

Individual days may be run with `-p rs-<day>`, and individual parts with `--bin rs-<day>-pt<n>`

## Dependencies

Although there are no hard dependencies for running the solution tests outside of cargo itself, there are a few things which may make it easier.

* `cargo-nextest` - Slightly nicer runner for executing tests, with color and summaries
* `carg-generate` - Used to generate daily templates, called through the use of `aoc.nu`, but also may be called manually through
  * `cargo generate --path templates/rust-template -f --name rs_$DAY`
* `hyperfine` - Convenient way to benchmark two executables against each other
