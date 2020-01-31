[![Build Status](https://dev.azure.com/immunant/rexpat/_apis/build/status/immunant.rexpat?branchName=master)](https://dev.azure.com/immunant/rexpat/_build/latest?definitionId=2&branchName=master)

# Rexpat: a libexpat compatible Rust crate

This project is a work-in-progress conversion from unsafe Rust transpiled directly from libexpat into safe, idiomatic Rust code. The initial transpilation and refactoring was done using [C2Rust](https://github.com/immunant/c2rust).

Do **not** use this in production (yet!), but help refactoring and rewriting is always welcome.

## Building

Requirements: Linux host with [`rustup`](https://rustup.rs/) installed. To run tests, you'll also need to install the requirements of libexpat (autoconf 2.58 or newer, make, and a recent C toolchain).

    $ git clone --recurse-submodules https://github.com/immunant/rexpat
    $ cd rexpat && cargo build

## Testing
Unit testing:    
    
    $ cargo test

Download the W2C XML test suite to `/tmp/libexpat/xml-test-suite` and run

    $ ./test_w2c.sh 

To perform additional testing. 

## Benchmarking

*NOTE:* Requires that you build `$REXPAT_ROOT/upstream/expat/tests/benchmark/benchmark` first. See steps and requirements [here](https://github.com/libexpat/libexpat/). You must also have `python3` (Python 3.6 or later) in your path.

    $ ./src/tests/bench_c_vs_rust.py

## Goals

- Provide an ABI-compatible drop-in replacement for `libexpat`
- Avoid memory-corruption vulnerabilities via Rust's safety guarantees.
- Perform on par with the `libexpat` in time and space.


## License

`rexpat` is free software licensed similarly to `libexpat`. You may copy, distribute, and modify it under the terms of the License contained in the file [COPYING](https://github.com/immunant/rexpat/blob/master/COPYING) distributed with this package. This license is the same as the MIT/X Consortium license.
