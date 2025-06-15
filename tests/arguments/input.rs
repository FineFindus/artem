///! Test the input argument, including url and file inputs

pub mod input {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::common::load_correct_file;

    #[test]
    fn input_does_not_exist() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("test/non-existing/file");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("does not exist"));
    }

    #[test]
    fn input_is_dir() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("test/");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("does not exist"));
    }

    #[test]
    fn correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("assets/images/standard_test_img.png");
        // check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }

    #[test]
    #[cfg(not(feature = "web_image"))]
    fn url_disabled_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg(
            "https://raw.githubusercontent.com/FineFindus/artem/master/assets/images/standard_test_img.png",
        );
        // check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .failure()
            .stderr(predicate::str::starts_with("[ERROR] File https://raw.githubusercontent.com/FineFindus/artem/master/assets/images/standard_test_img.png does not exist"));
    }

    #[test]
    #[cfg(not(feature = "web_image"))]
    fn help_shows_correct_info_no_url() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("--help");
        cmd.assert().success().stdout(predicate::str::contains(
            // only test beginning, since different formatting would break the rest
            "Paths to the target image. The original image is NOT altered.",
        ));
    }

    #[test]
    fn multiple_input_is_false() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args([
            "assets/images/standard_test_img.png",
            "examples/non_existing.jpg",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("does not exist"));
    }

    #[test]
    fn multiple_correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args([
            "assets/images/standard_test_img.png",
            "assets/images/standard_test_img.png",
        ]);

        let mut ascii_img = String::new();
        // add img twice, since it was given twice as an input
        ascii_img.push_str(&load_correct_file());
        ascii_img.push('\n');
        ascii_img.push_str(&load_correct_file());
        // check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(ascii_img));
    }
}

#[cfg(feature = "web_image")]
pub mod url_input {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::common::load_correct_file;

    #[test]
    fn input_does_not_exist() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("https://example.com/no.png");
        cmd.assert().failure().stderr(predicate::str::contains(
            "[ERROR] Failed to load image bytes from https://example.com/no.png",
        ));
    }

    #[test]
    fn correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        // use example abraham lincoln image from github repo
        cmd.arg(
            "https://raw.githubusercontent.com/FineFindus/artem/master/assets/images/standard_test_img.png",
        );
        // check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }

    #[test]
    fn multiple_input_is_false() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args([
            "https://example.com/no-image.jpg",
            "https://example.com/no.png",
        ]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "[ERROR] Failed to load image bytes from https://example.com/no-image.jpg",
        ));
    }

    #[test]
    fn multiple_correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args([
            "https://raw.githubusercontent.com/FineFindus/artem/master/assets/images/standard_test_img.png",
            "https://raw.githubusercontent.com/FineFindus/artem/master/assets/images/standard_test_img.png",
        ]);

        let mut ascii_img = String::new();
        // add img twice, since it was given twice as an input
        ascii_img.push_str(&load_correct_file());
        ascii_img.push('\n');
        ascii_img.push_str(&load_correct_file());
        // check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(ascii_img));
    }

    #[test]
    #[cfg(feature = "web_image")]
    fn help_shows_correct_info() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("--help");
        cmd.assert().success().stdout(predicate::str::contains(
            // only test beginning, since different formatting would break the rest
            "Paths or URLs to the target image. If the input is an URL, the image is",
        ));
    }
}
