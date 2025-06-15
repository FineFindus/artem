pub mod output_file {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::{fs, process::Command};

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png").arg("-o");
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: a value is required for '--output <output-file>' but none was supplied",
        ));
    }

    #[test]
    // windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn file_is_ansi() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-o", "/tmp/ascii.ans"]);
        // only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 2105 bytes to /tmp/ascii.ans",
        ));
        // delete output file
        fs::remove_file("/tmp/ascii.ans").unwrap();
    }

    #[test]
    // windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn file_is_html() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-o", "/tmp/ascii.html"]);
        // only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 62626 bytes to /tmp/ascii.html",
        ));
        // delete output file
        fs::remove_file("/tmp/ascii.html").unwrap();
    }

    #[test]
    // windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn file_plain_text() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-o", "/tmp/test.txt"]);
        // only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 2105 bytes to /tmp/test.txt",
        ));
        // delete output file
        fs::remove_file("/tmp/test.txt").unwrap();
    }
}

pub mod verbosity {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--verbose");
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: a value is required for '--verbose <verbosity>' but none was supplied",
        ));
    }

    #[test]
    fn arg_info() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--verbose", "info"]);
        // only check first line
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("INFO"));
    }

    #[test]
    fn arg_debug() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--verbose", "debug"]);
        // only check first line
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("DEBUG"));
    }

    #[test]
    fn arg_error() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.nonexisting") // this causes a fatal error
            .args(["--verbose", "error"]);
        // only check first line
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("ERROR"));
    }
}
