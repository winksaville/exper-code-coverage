# Experiment with code coverage

![ci-stable](https://github.com/winksaville/exper-code-coverage/actions/workflows/ci-stable.yml/badge.svg)
![ci-nightly](https://github.com/winksaville/exper-code-coverage/actions/workflows/ci-nightly.yml/badge.svg)
[![codecov](https://codecov.io/gh/winksaville/exper-code-coverage/branch/main/graph/badge.svg?token=cowZtK1KK1)](https://codecov.io/gh/winksaville/exper-code-coverage)

Experiment with code coverage and see how well [LLVM source-based code coverage](https://rustc-dev-guide.rust-lang.org/llvm-coverage-instrumentation.html) works.
> Note: I believe the library with `if_short_circuit_and()` and `short_circuit_and()`
> shows that `branch coverage` as tracked in [rust issue #79649](https://github.com/rust-lang/rust/issues/79649).
> I've provided a couple comments to that effect [here](https://github.com/rust-lang/rust/issues/79649#issuecomment-1214483770)
> and [here](https://github.com/rust-lang/rust/issues/79649#issuecomment-1215764155).

Short answer seems to be that llvm-cov is better than tarpaulin as I
never see 100% for trapaulin.  But llvm-cov is not perfect. I've
created two equivalent functions; if_short_circuit_and() and short_circuit_and().

When using llvm-cov on the tests for if_short_circuit_and() 100% isn't reported
until all branches are tested. Exactly what I'd expect.

But in short_circuit_and() 100% can be achived with either of the
last two tests, which is not ideal.

Here are the libraries:
```
wink@3900x 22-08-15T00:05:12.879Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/if_short_circuit_and.rs 
     1  pub fn if_short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      if a < b {
     3          c < d
     4      } else {
     5          false
     6      }
     7  }
     8
     9  #[cfg(test)]
    10  mod tests {
    11      use super::*;
    12
    13      // llvm-cov 100% if all are enabled.
    14
    15      // llvm-cov 85.71% if only this is enabled.
    16      #[test]
    17      pub fn test_short_circuit_and_first_false() {
    18          assert_eq!(if_short_circuit_and(20, 10, 30, 40), false);
    19      }
    20
    21      // llvm-cov 85.71% if only this is enabled
    22      #[test]
    23      pub fn test_short_circuit_and_both_true() {
    24          assert_eq!(if_short_circuit_and(10, 20, 30, 40), true);
    25      }
    26  }
wink@3900x 22-08-15T00:05:19.936Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/short_circuit_and.rs 
     1  pub fn short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      a < b && c < d
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      // llvm-cov 100% if all are enabled.
    10
    11      // llvm-cov 83.33% if only this is enabled
    12      #[test]
    13      pub fn test_short_circuit_and_first_false() {
    14          assert_eq!(short_circuit_and(20, 10, 30, 40), false);
    15      }
    16
    17      // llvm-cov 100% if only this is enabled
    18      #[test]
    19      pub fn test_short_circuit_and_both_true() {
    20          assert_eq!(short_circuit_and(10, 20, 30, 40), true);
    21      }
    22  }
```

# Installation

Install cargo-llvm-cov and cargo-tarpaulin:
```
cargo install cargo-llvm-cov
cargo install cargo-tarpaulin
```

## LLMM Source-Based Code Coverage

With all tests enable we see 100% coverage:
```
wink@3900x 22-08-15T00:05:24.119Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo llvm-cov
   Compiling exper-code-coverage v0.2.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-f4d7ca3a8a720d31)

running 4 tests
test if_short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test if_short_circuit_and::tests::test_short_circuit_and_first_false ... ok
test short_circuit_and::tests::test_short_circuit_and_first_false ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-c502fcb0cc109ca3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-55e882f44094d43e)

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
if_short_circuit_and.rs            10                 0   100.00%           5                 0   100.00%          13                 0   100.00%           0                 0         -
lib.rs                              1                 0   100.00%           1                 0   100.00%           1                 0   100.00%           0                 0         -
main.rs                             2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
short_circuit_and.rs                9                 0   100.00%           5                 0   100.00%          11                 0   100.00%           0                 0         -
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                              22                 0   100.00%          13                 0   100.00%          29                 0   100.00%           0                 0         -
```

If I use only the first test of the two libraries I get 77.78% coverage of regions:
```
wink@3900x 22-08-15T00:15:31.408Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/if_short_circuit_and.rs 
     1  pub fn if_short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      if a < b {
     3          c < d
     4      } else {
     5          false
     6      }
     7  }
     8
     9  #[cfg(test)]
    10  mod tests {
    11      use super::*;
    12
    13      // llvm-cov 100% if all are enabled.
    14
    15      // llvm-cov 85.71% if only this is enabled.
    16      #[test]
    17      pub fn test_short_circuit_and_first_false() {
    18          assert_eq!(if_short_circuit_and(20, 10, 30, 40), false);
    19      }
    20
    21      //// llvm-cov 85.71% if only this is enabled
    22      //#[test]
    23      //pub fn test_short_circuit_and_both_true() {
    24      //    assert_eq!(if_short_circuit_and(10, 20, 30, 40), true);
    25      //}
    26  }
wink@3900x 22-08-15T00:15:33.328Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/short_circuit_and.rs 
     1  pub fn short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      a < b && c < d
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      // llvm-cov 100% if all are enabled.
    10
    11      // llvm-cov 83.33% if only this is enabled
    12      #[test]
    13      pub fn test_short_circuit_and_first_false() {
    14          assert_eq!(short_circuit_and(20, 10, 30, 40), false);
    15      }
    16
    17      //// llvm-cov 100% if only this is enabled
    18      //#[test]
    19      //pub fn test_short_circuit_and_both_true() {
    20      //    assert_eq!(short_circuit_and(10, 20, 30, 40), true);
    21      //}
    22  }
wink@3900x 22-08-15T00:15:34.639Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo llvm-cov
   Compiling exper-code-coverage v0.2.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-f4d7ca3a8a720d31)

running 2 tests
test if_short_circuit_and::tests::test_short_circuit_and_first_false ... ok
test short_circuit_and::tests::test_short_circuit_and_first_false ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-c502fcb0cc109ca3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-55e882f44094d43e)

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
if_short_circuit_and.rs             7                 1    85.71%           3                 0   100.00%           9                 1    88.89%           0                 0         -
lib.rs                              1                 0   100.00%           1                 0   100.00%           1                 0   100.00%           0                 0         -
main.rs                             2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
short_circuit_and.rs                6                 1    83.33%           3                 0   100.00%           7                 0   100.00%           0                 0         -
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                              16                 2    87.50%           9                 0   100.00%          21                 1    95.24%           0                 0         -
```

But if I enable `test_short_circuit_both_true()` on both libraries we get 100%
for `short_circuit_and.rs` and 85.71% for `if_short_circuit_and.rs`.
Not the ideal result, IMHO:
```
wink@3900x 22-08-15T00:10:59.494Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/if_short_circuit_and.rs 
     1  pub fn if_short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      if a < b {
     3          c < d
     4      } else {
     5          false
     6      }
     7  }
     8
     9  #[cfg(test)]
    10  mod tests {
    11      use super::*;
    12
    13      // llvm-cov 100% if all are enabled.
    14
    15      // llvm-cov 85.71% if only this is enabled.
    16      //#[test]
    17      //pub fn test_short_circuit_and_first_false() {
    18      //    assert_eq!(if_short_circuit_and(20, 10, 30, 40), false);
    19      //}
    20
    21      // llvm-cov 85.71% if only this is enabled
    22      #[test]
    23      pub fn test_short_circuit_and_both_true() {
    24          assert_eq!(if_short_circuit_and(10, 20, 30, 40), true);
    25      }
    26  }
wink@3900x 22-08-15T00:11:00.865Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/short_circuit_and.rs 
     1  pub fn short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      a < b && c < d
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      // llvm-cov 100% if all are enabled.
    10
    11      $ cat -n src/short_circuit_and.rs 
     1  pub fn short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      a < b && c < d
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      // llvm-cov 100% if all are enabled.
    10
    11      //// llvm-cov 83.33% if only this is enabled
    12      //#[test]
    13      //pub fn test_short_circuit_and_first_false() {
    14      //    assert_eq!(short_circuit_and(20, 10, 30, 40), false);
    15      //}
    16
    17      // llvm-cov 100% if only this is enabled
    18      #[test]
    19      pub fn test_short_circuit_and_both_true() {
    20          assert_eq!(short_circuit_and(10, 20, 30, 40), true);
    21      }
    22  }
wink@3900x 22-08-15T00:11:02.440Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo llvm-cov
   Compiling exper-code-coverage v0.2.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-f4d7ca3a8a720d31)

running 2 tests
test if_short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test short_circuit_and::tests::test_short_circuit_and_both_true ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/llvm-cov-target/debug/deps/exper_code_coverage-c502fcb0cc109ca3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-55e882f44094d43e)

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
if_short_circuit_and.rs             7                 1    85.71%           3                 0   100.00%           9                 1    88.89%           0                 0         -
lib.rs                              1                 0   100.00%           1                 0   100.00%           1                 0   100.00%           0                 0         -
main.rs                             2                 0   100.00%           2                 0   100.00%           4                 0   100.00%           0                 0         -
short_circuit_and.rs                6                 0   100.00%           3                 0   100.00%           7                 0   100.00%           0                 0         -
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                              16                 1    93.75%           9                 0   100.00%          21                 1    95.24%           0                 0         -
```

## Tarpaulin Code Coverage

Tarpaulin doesn't do very well with all tests enabled and
only reports `17.94% coverage, 73/407 lines covered`:
```
wink@3900x 22-08-15T19:06:07.086Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/if_short_circuit_and.rs 
     1  pub fn if_short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      if a < b {
     3          c < d
     4      } else {
     5          false
     6      }
     7  }
     8
     9  #[cfg(test)]
    10  mod tests {
    11      use super::*;
    12
    13      // llvm-cov 100% if all are enabled.
    14
    15      // llvm-cov 85.71% if only this is enabled.
    16      #[test]
    17      pub fn test_short_circuit_and_first_false() {
    18          assert_eq!(if_short_circuit_and(20, 10, 30, 40), false);
    19      }
    20
    21      // llvm-cov 85.71% if only this is enabled
    22      #[test]
    23      pub fn test_short_circuit_and_both_true() {
    24          assert_eq!(if_short_circuit_and(10, 20, 30, 40), true);
    25      }
    26  }
wink@3900x 22-08-15T19:06:11.866Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n src/short_circuit_and.rs 
     1  pub fn short_circuit_and(a: i32, b: i32, c: i32, d: i32) -> bool {
     2      a < b && c < d
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      // llvm-cov 100% if all are enabled.
    10
    11      // llvm-cov 83.33% if only this is enabled
    12      #[test]
    13      pub fn test_short_circuit_and_first_false() {
    14          assert_eq!(short_circuit_and(20, 10, 30, 40), false);
    15      }
    16
    17      // llvm-cov 100% if only this is enabled
    18      #[test]
    19      pub fn test_short_circuit_and_both_true() {
    20          assert_eq!(short_circuit_and(10, 20, 30, 40), true);
    21      }
    22  }
wink@3900x 22-08-15T19:06:19.721Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo tarpaulin
Aug 15 12:06:24.499  INFO cargo_tarpaulin::config: Creating config
Aug 15 12:06:24.514  INFO cargo_tarpaulin: Running config test_config
Aug 15 12:06:24.514  INFO cargo_tarpaulin: Running Tarpaulin
Aug 15 12:06:24.514  INFO cargo_tarpaulin: Building project
Aug 15 12:06:24.514  INFO cargo_tarpaulin::cargo: Cleaning project
   Compiling memchr v2.5.0
   Compiling autocfg v1.1.0
   Compiling libc v0.2.131
   Compiling predicates-core v1.0.3
   Compiling doc-comment v0.3.3
   Compiling regex-syntax v0.6.27
   Compiling either v1.7.0
   Compiling difflib v0.4.0
   Compiling normalize-line-endings v0.3.0
   Compiling termtree v0.2.4
   Compiling lazy_static v1.4.0
   Compiling regex-automata v0.1.10
   Compiling exper-code-coverage v0.2.0 (/home/wink/prgs/rust/myrepos/exper-code-coverage)
   Compiling itertools v0.10.3
   Compiling predicates-tree v1.0.5
   Compiling num-traits v0.2.15
   Compiling aho-corasick v0.7.18
   Compiling bstr v0.2.17
   Compiling wait-timeout v0.2.0
   Compiling float-cmp v0.9.0
   Compiling regex v1.6.0
   Compiling predicates v2.1.1
   Compiling assert_cmd v2.0.4
    Finished test [unoptimized + debuginfo] target(s) in 3.25s
Aug 15 12:06:27.837  INFO cargo_tarpaulin::process_handling::linux: Launching test
Aug 15 12:06:27.837  INFO cargo_tarpaulin::process_handling: running /home/wink/prgs/rust/myrepos/exper-code-coverage/target/debug/deps/exper_code_coverage-095edf15d68d9ffe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Aug 15 12:06:28.087  INFO cargo_tarpaulin::process_handling::linux: Launching test
Aug 15 12:06:28.087  INFO cargo_tarpaulin::process_handling: running /home/wink/prgs/rust/myrepos/exper-code-coverage/target/debug/deps/exper_code_coverage-de097368a0c20a9c

running 4 tests
test if_short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test if_short_circuit_and::tests::test_short_circuit_and_first_false ... ok
test short_circuit_and::tests::test_short_circuit_and_both_true ... ok
test short_circuit_and::tests::test_short_circuit_and_first_false ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Aug 15 12:06:28.339  INFO cargo_tarpaulin::process_handling::linux: Launching test
Aug 15 12:06:28.339  INFO cargo_tarpaulin::process_handling: running /home/wink/prgs/rust/myrepos/exper-code-coverage/target/debug/deps/cli-0c4944e0cd6836e7

running 1 test
test test_no_params ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.84s

Aug 15 12:06:29.831  INFO cargo_tarpaulin::report: Coverage Results:
|| Tested/Total Lines:
|| src/if_short_circuit_and.rs: 4/4 +0.00%
|| src/lib.rs: 65/399 +0.00%
|| src/main.rs: 2/2 +0.00%
|| src/short_circuit_and.rs: 2/2 +0.00%
|| 
17.94% coverage, 73/407 lines covered, +0% change in coverage
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
