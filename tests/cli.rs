use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

const APP_NAME: &str = "expr-code-coverage";

// If tarpaulin starts failing during github Actions disable
// by uncommenting #[cfg(not(tarpauling))]. This will reduce
// coverage as reported by tarpaulin but so be it!
//
// Note: execute "cargo tarpaulin --follow-exec" so these are run
#[test]
//#[cfg(not(tarpaulin))]
fn test_no_params() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    let expected_to_start_with: &str = "Hello, world!";

    cmd.assert()
        .code(predicate::eq(0))
        .stdout(predicate::str::starts_with(expected_to_start_with));

    Ok(())
}
