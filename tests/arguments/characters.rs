pub mod characters {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::common::load_correct_file;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("assets/images/standard_test_img.png").arg("-c");
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: a value is required for '--characters <characters>' but none was supplied",
        ));
    }

    #[test]
    fn arg_is_number() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("assets/images/standard_test_img.png").arg("-c 0.6");
        cmd.assert().success().stdout(predicate::str::starts_with(
            "..........0000000000000000000000000000000000.6666666666666666666666666..........",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("assets/images/standard_test_img.png")
            .args(["-c", "M0123-."]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "333333333311111111111111111122222222222222223-----------------........3333333333",
        ));
    }

    #[test]
    fn arg_preset_0_short_s() {
        for arg in ["short", "s", "0"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("assets/images/standard_test_img.png")
                .args(["-c", arg]);
            //only check first line
            cmd.assert().success().stdout(predicate::str::starts_with(
                "aaaaaaaaaa6666666665555555542222222211111111b:::::::+=========,,,,,,,,aaaaaaaaaa",
            ));
        }
    }

    #[test]
    fn arg_preset_1_flat_f() {
        for arg in ["flat", "f", "1"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("assets/images/standard_test_img.png")
                .args(["-c", arg]);
            //only check first line
            cmd.assert()
                .success()
                .stdout(predicate::str::starts_with(load_correct_file()));
        }
    }

    #[test]
    fn arg_preset_2_long_l() {
        for arg in ["long", "l", "2"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("assets/images/standard_test_img.png")
                .args(["-c", arg]);
            //only check first line
            cmd.assert().success().stdout(predicate::str::starts_with(
                r"\\\\\\\\\\ZZZZZZZZOQQQQQQQQJzzzzzzzzuuuuuuuu)++++++++>>>>>>>>i::::::::\\\\\\\\\\",
            ));
        }
    }
}
