pub mod scale {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("assets/images/standard_test_img.png")
            .arg("--ratio");
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: a value is required for '--ratio <scale>' but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        // should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--ratio", "string"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: invalid value 'string' for '--ratio <scale>': invalid float literal",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        // should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--ratio", "-6"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: unexpected argument '-6' found",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        // should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--ratio", &f64::MAX.to_string()]);
        cmd.assert().success().stdout(predicate::str::starts_with(
            "::::::::::OOOOOOOOkkkkkkkkkxddddddddoooooooo;.................        ::::::::::",
        ));
    }

    #[test]
    fn arg_is_zero() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        // should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--ratio", "0"]);
        cmd.assert().success().stdout(predicate::str::starts_with(
            "::::::::::OOOOOOOOkkkkkkkkkxddddddddoooooooo;.................        ::::::::::",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--ratio", "0.75"]);
        // only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "::::::::::OOOOOOOOkkkkkkkkkxddddddddoooooooo;.................        ::::::::::",
        ));
    }
}
