pub mod flip_x {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--flipX", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--flipX");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "::::::::::        .................;ooooooooddddddddxkkkkkkkkkOOOOOOOO::::::::::",
        ));
    }
}

pub mod flip_y {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--flipY", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--flipY");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "..........            cWWWWWWWWWWWWWWWWW                              ..........",
        ));
    }
}

pub mod flip_x_y {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--flipY", "--flipX"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "..........                              WWWWWWWWWWWWWWWWWc            ..........",
        ));
    }
}

pub mod outline {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--outline", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--outline");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "         ll       .        :       .       ;x       .        :       ll         ",
        ));
    }
}

pub mod hysteresis {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn outline_is_required() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--hysteresis");
        cmd.assert()
            .failure()
            .stderr(predicate::str::starts_with(
                "error: The following required arguments were not provided:",
            ))
            .stderr(predicate::str::contains("--outline"));
    }

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--outline", "--hysteresis", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--outline", "--hys"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "         ll                O               ;x                O       ll         ",
        ));
    }
}

pub mod border {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["--border", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .arg("--border");
        //only check first line
        cmd.assert()
            .success().stdout(predicate::str::starts_with(
                "╔══════════════════════════════════════════════════════════════════════════════╗",
            ))
            .success().stdout(predicate::str::ends_with(
                "╚══════════════════════════════════════════════════════════════════════════════╝\n",
            ));
    }
}
