# Experiment with code coverage

![ci-stable](https://github.com/winksaville/exper-code-coverage/actions/workflows/ci-stable.yml/badge.svg)
![ci-nightly](https://github.com/winksaville/exper-code-coverage/actions/workflows/ci-nightly.yml/badge.svg)
[![codecov](https://codecov.io/gh/winksaville/exper-code-coverage/branch/main/graph/badge.svg?token=cowZtK1KK1)](https://codecov.io/gh/winksaville/exper-code-coverage)

Experiment with code coverage and see how well [LLVM source-based code coverage](https://rustc-dev-guide.rust-lang.org/llvm-coverage-instrumentation.html) works.

I'm using cargo-llvm-cov and tarpaulin to install:
```
cargo install cargo-llvm-cov
cargo install cargo-tarpaulin
```

## LLMM Source-Based Code Coverage

Currently "100%"

```
wink@3900x:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo llvm-cov
   Compiling exper-code-coverage v0.1.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
    Finished test [unoptimized + debuginfo] target(s) in 0.79s
     Running unittests (target/llvm-cov-target/debug/deps/exper_code_coverage-ae7fad6d44ba68fc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-45ae8237db2683e9)

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                                                        Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/home/wink/prgs/rust/myrepos/exper-code-coverage/src/main.rs           2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                                 2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
wink@3900x:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo llvm-cov
   Compiling exper-code-coverage v0.1.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
    Finished test [unoptimized + debuginfo] target(s) in 0.79s
     Running unittests (target/llvm-cov-target/debug/deps/exper_code_coverage-ae7fad6d44ba68fc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-45ae8237db2683e9)

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                                                        Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/home/wink/prgs/rust/myrepos/exper-code-coverage/src/main.rs           2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                        
```

## Tarpaulin Code Coverage

Currently "100%" 2/2. 

```
wink@3900x:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo tarpaulin
Apr 08 11:33:14.839  INFO cargo_tarpaulin::config: Creating config
Apr 08 11:33:14.852  INFO cargo_tarpaulin: Running config test_config
Apr 08 11:33:14.852  INFO cargo_tarpaulin: Running Tarpaulin
Apr 08 11:33:14.852  INFO cargo_tarpaulin: Building project
Apr 08 11:33:14.852  INFO cargo_tarpaulin::cargo: Cleaning project
   Compiling memchr v2.4.1
   Compiling autocfg v1.1.0
   Compiling libc v0.2.122
   Compiling either v1.6.1
   Compiling regex-syntax v0.6.25
   Compiling doc-comment v0.3.3
   Compiling predicates-core v1.0.3
   Compiling normalize-line-endings v0.3.0
   Compiling lazy_static v1.4.0
   Compiling difflib v0.4.0
   Compiling termtree v0.2.4
   Compiling regex-automata v0.1.10
   Compiling exper-code-coverage v0.1.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
   Compiling itertools v0.10.3
   Compiling predicates-tree v1.0.5
   Compiling num-traits v0.2.14
   Compiling aho-corasick v0.7.18
   Compiling bstr v0.2.17
   Compiling wait-timeout v0.2.0
   Compiling float-cmp v0.9.0
   Compiling regex v1.5.5
   Compiling predicates v2.1.1
   Compiling assert_cmd v2.0.4
    Finished test [unoptimized + debuginfo] target(s) in 3.51s
Apr 08 11:33:18.446  INFO cargo_tarpaulin::process_handling::linux: Launching test
Apr 08 11:33:18.446  INFO cargo_tarpaulin::process_handling: running /home/wink/prgs/rust/myrepos/exper-code-coverage/target/debug/deps/exper_code_coverage-d676e043ae6e11c6

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Apr 08 11:33:18.579  INFO cargo_tarpaulin::process_handling::linux: Launching test
Apr 08 11:33:18.579  INFO cargo_tarpaulin::process_handling: running /home/wink/prgs/rust/myrepos/exper-code-coverage/target/debug/deps/cli-d86d8363bb9f499e

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.43s

Apr 08 11:33:20.827  INFO cargo_tarpaulin::report: Coverage Results:
|| Tested/Total Lines:
|| src/main.rs: 2/2 +50.00%
|| 
100.00% coverage, 2/2 lines covered, +50% change in coverage

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
