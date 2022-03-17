use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::fs::{self};
// Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn full_file_compare_no_args() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("examples/abraham_lincoln.jpg");

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
fn full_file_compare_border() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("examples/abraham_lincoln.jpg").arg("--border");

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln_border.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}
