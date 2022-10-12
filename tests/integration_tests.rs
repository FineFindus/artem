use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::fs::{self};
// Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn full_file_compare_no_args() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("assets/images/standard_test_img.png");

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
#[cfg(feature = "web_image")]
fn full_file_compare_url() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg(
        "https://raw.githubusercontent.com/FineFindus/artem/master/assets/images/standard_test_img.png",
    );

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
fn full_file_compare_border() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("assets/images/standard_test_img.png")
        .arg("--border");

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_border.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
fn full_file_compare_outline() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    //this example image is not the best case for the outline, since its already grayscale, and the person is a lot darker than the background
    cmd.arg("assets/images/standard_test_img.png")
        .arg("--outline");

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_outline.txt").unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
fn full_file_compare_border_outline() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    //this example image is not the best case for the outline, since its already grayscale, and the person is a lot darker than the background
    cmd.arg("assets/images/standard_test_img.png")
        .arg("--outline")
        .arg("--border");

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_border_outline.txt")
            .unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
fn full_file_compare_outline_hysteresis() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    //this example image is not the best case for the outline, since its already grayscale, and the person is a lot darker than the background
    cmd.arg("assets/images/standard_test_img.png")
        .args(["--outline", "--hysteresis"]);

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_outline_hysteresis.txt")
            .unwrap(); //ignore errors
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(desired_output));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn full_file_compare_html() {
    let mut cmd = Command::cargo_bin("artem").unwrap();

    cmd.arg("assets/images/standard_test_img.png")
        .args(["-o", "/tmp/ascii.html"]);

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 62674 bytes to /tmp/ascii.html",
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

    cmd.arg("assets/images/standard_test_img.png")
        .args(["-o", "/tmp/ascii.html", "--border"]);

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_border.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 61712 bytes to /tmp/ascii.html",
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

    cmd.arg("assets/images/standard_test_img.png")
        .args(["-o", "/tmp/ascii.html", "--outline"]);

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_outline.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 19834 bytes to /tmp/ascii.html",
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

    cmd.arg("assets/images/standard_test_img.png")
        .args(["-o", "/tmp/ascii.html", "--background"]);

    //load file contents to compare
    let desired_output =
        fs::read_to_string("assets/standard_test_img/standard_test_img_background.html").unwrap(); //ignore errors
    cmd.assert().success().stdout(predicate::str::contains(
        "Written 100242 bytes to /tmp/ascii.html",
    ));

    let file_output = fs::read_to_string("/tmp/ascii.html").unwrap(); //ignore errors

    //delete output file
    fs::remove_file("/tmp/ascii.html").unwrap();

    assert_eq!(desired_output, file_output);
}
