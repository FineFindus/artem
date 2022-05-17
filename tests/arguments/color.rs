pub mod invert {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--invert", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--invert");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "dddddddddd'''''''',,,,,,,,,;::::::::ccccccccx00000000KKKKKKKKKNNNNNNNNdddddddddd",
        ));
    }
}

pub mod no_color {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::common::load_correct_file;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--no-color", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_background() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--no-color", "--background"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--no-color' cannot be used with '--background'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--no-color");
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod background_color {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::common::load_correct_file;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--background", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_no_color() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--background", "--no-color"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--background' cannot be used with '--no-color'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--background");
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}
