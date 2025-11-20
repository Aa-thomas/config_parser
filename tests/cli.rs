use assert_cmd::cargo::cargo_bin_cmd;

use predicates::prelude::*;

#[test]
fn test_read_json_value() -> anyhow::Result<()> {
    let mut cmd = cargo_bin_cmd!("config_parser");
    cmd.arg("--document")
        .arg(".fixtures/jfix.json")
        .arg("read")
        .arg("nested.arr_in_obj[1].y[2]");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("30"));

    Ok(())
}

#[test]
fn test_read_toml_value() -> anyhow::Result<()> {
    let mut cmd = cargo_bin_cmd!("config_parser");
    cmd.arg("--document")
        .arg(".fixtures/tfix.toml")
        .arg("read")
        .arg("top.str");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hi"));

    Ok(())
}

#[test]
fn test_read_file_not_found() -> anyhow::Result<()> {
    let mut cmd = cargo_bin_cmd!("config_parser");
    cmd.arg("--document")
        .arg("no/such/file.json")
        .arg("read")
        .arg("a.b");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}
