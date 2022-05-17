pub mod size {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("assets/images/standard_test_img.png").arg("-s");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png")
            .arg("-s string");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_float() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png").arg("-s 0.6");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png").arg("-s -6");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png")
            .arg(format!("-s {}", u32::MAX));
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_conflict_width() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-s", "75"])
            .arg("-w");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--width'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-s", "75"])
            .arg("-h");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--height'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-s", "75"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            ":::::::::dOOOOOOOkkkkkkkkxdddddddoooooooo:................       ':::::::::",
        ));
    }
}

pub mod width {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-w", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("-w")
            .args(["-s", "75"]);
        //should panic when trying using both args
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--size <size>'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("assets/images/standard_test_img.png")
            .arg("-w")
            .arg("-h");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--height'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--width");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().success().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod height {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-h", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("-h")
            .args(["-s", "75"]);
        //should panic when trying using both args
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--size <size>'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("assets/images/standard_test_img.png")
            .arg("-h")
            .arg("-w");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--width'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--height");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().success().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}
