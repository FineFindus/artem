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

#[test]
fn full_file_compare_outline() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    //this example image is not the best case for the outline, since its already grayscale, and the person is a lot darker than the background
    cmd.arg("examples/abraham_lincoln.jpg").arg("--outline");

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln_outline.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
fn full_file_compare_border_outline() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    //this example image is not the best case for the outline, since its already grayscale, and the person is a lot darker than the background
    cmd.arg("examples/abraham_lincoln.jpg")
        .arg("--outline")
        .arg("--border");

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln_border_outline.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn full_file_compare_html() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("examples/abraham_lincoln.jpg")
        .args(["-o", "/tmp/ascii.html"]);

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 133621 bytes to /tmp/ascii.html",
    ));

    let file_output = fs::read_to_string("/tmp/ascii.html").unwrap(); //ignore errors

    //delete output file
    fs::remove_file("/tmp/ascii.html").unwrap();

    assert_eq!(desired_output, file_output);
}

#[test]
#[cfg(not(target_os = "windows"))]
fn full_file_compare_html_border() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("examples/abraham_lincoln.jpg")
        .args(["-o", "/tmp/ascii.html", "--border"]);

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln_border.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 128149 bytes to /tmp/ascii.html",
    ));

    let file_output = fs::read_to_string("/tmp/ascii.html").unwrap(); //ignore errors

    //delete output file
    fs::remove_file("/tmp/ascii.html").unwrap();

    assert_eq!(desired_output, file_output);
}

#[test]
#[cfg(not(target_os = "windows"))]
fn full_file_compare_html_outline() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("examples/abraham_lincoln.jpg")
        .args(["-o", "/tmp/ascii.html", "--outline"]);

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln_outline.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 133621 bytes to /tmp/ascii.html",
    ));

    let file_output = fs::read_to_string("/tmp/ascii.html").unwrap(); //ignore errors

    //delete output file
    fs::remove_file("/tmp/ascii.html").unwrap();

    assert_eq!(desired_output, file_output);
}

#[test]
#[cfg(not(target_os = "windows"))]
fn full_file_compare_html_background_color() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("examples/abraham_lincoln.jpg")
        .args(["-o", "/tmp/ascii.html", "--background"]);

    //load file contents to compare
    let desired_output = fs::read_to_string("assets/abraham_lincoln_background.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 173221 bytes to /tmp/ascii.html",
    ));

    let file_output = fs::read_to_string("/tmp/ascii.html").unwrap(); //ignore errors

    //delete output file
    fs::remove_file("/tmp/ascii.html").unwrap();

    assert_eq!(desired_output, file_output);
}
