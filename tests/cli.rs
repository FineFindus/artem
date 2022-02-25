pub mod input {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run programs

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

        cmd.arg("examples/abraham_lincoln.jpg");
        //check only the first line, the rest is likely to be correct as well
        cmd.assert().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod density {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("-c");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--characters <density>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_number() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-c 0.6");
        cmd.assert().stdout(predicate::str::starts_with(
            "                       00000 00000000000000000000000000000000000000000          ",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-c", "M0123-."]);
        //only check first line
        cmd.assert().stdout(predicate::str::starts_with(
            "MMMMMMMMMM0000000000000000000000000000010111111111111111111111001100000000000000",
        ));
    }

    #[test]
    fn arg_preset_0_short_s() {
        for arg in ["short", "s", "0"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().stdout(predicate::str::starts_with(
                "####WWWWW$$$$$9999998877767777776666676665554456555555433335566656677788988899$$",
            ));
        }
    }

    #[test]
    fn arg_preset_1_flat_f() {
        for arg in ["flat", "f", "1"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().stdout(predicate::str::starts_with(
                "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
            ));
        }
    }

    #[test]
    fn arg_preset_2_long_l() {
        for arg in ["long", "l", "2"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().stdout(predicate::str::starts_with(
                "W&&WMMM##*aaaaahaahkbdpqwmwqpqwwmmmmmwmZm00OQQ0Z000000CUJJC0OZmm0Zmwqqdbkdbbkhao",
            ));
        }
    }
}

pub mod size {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("-s");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--size <size>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s string");
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_float() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s 0.6");
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s -6");
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg(format!("-s {}", u32::MAX));
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_conflict_width() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-s", "75"])
            .arg("-w");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--width'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-s", "75"])
            .arg("-h");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--height'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").args(["-s", "75"]);
        //only check first line
        cmd.assert().stdout(predicate::str::starts_with(
            "WWWNNNNNXXXXXXKXXKKK000OO000OOOOOO0OOOkkkkkOkkkkkkxxxxkkOOOkOO000KKKKKKXXXX",
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
        cmd.arg("examples/abraham_lincoln.jpg").args(["-w", "123"]);
        cmd.assert().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg("-w")
            .args(["-s", "75"]);
        //should panic when trying using both args
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--size <size>'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg").arg("-w").arg("-h");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--height'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--width");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().stdout(predicate::str::starts_with(
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
        cmd.arg("examples/abraham_lincoln.jpg").args(["-h", "123"]);
        cmd.assert().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg("-h")
            .args(["-s", "75"]);
        //should panic when trying using both args
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--size <size>'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg").arg("-h").arg("-w");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--width'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--height");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod scale {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("--ratio");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--ratio <scale>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "string"]);
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with ratio input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "-6"]);
        cmd.assert().stderr(predicate::str::starts_with(
            "error: Found argument '-6' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", f64::MAX.to_string().as_str()]);
        cmd.assert().stdout(predicate::str::starts_with(
            "NWWNNNNXXXXXKKKKXXKKK0000OO00OOOOOOkkOOkkxkkxxkkkkkkkkxxdddkkOOOkOOO00KKK00KKKXX",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "0.75"]);
        //only check first line
        cmd.assert().stdout(predicate::str::starts_with(
            "NWWNNNNNXXXXKKKKKXKKK0000OO000OOOOOOOOOkkxkkxxkOkkkkkkxxddxkkOOOkOOO00KKK00KKKXX",
        ));
    }
}

pub mod thread {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("--thread");
        cmd.assert().stderr(predicate::str::contains(
            "The argument '--thread <threads>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "string"]);
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with thread input value",
        ));
    }

    #[test]
    fn arg_is_float() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "0.6"]);
        cmd.assert().stderr(predicate::str::contains(
            "Could not work with thread input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "-6"]);
        cmd.assert().stderr(predicate::str::contains(
            "error: Found argument '-6' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", u32::MAX.to_string().as_str()]);
        //since its clamped, it should return the normal img
        cmd.assert().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "3"]);
        //only check first line
        cmd.assert().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod output_file {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::{fs, process::Command};

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("-o");
        cmd.assert().stderr(predicate::str::starts_with(
            "error: The argument '--output <output-file>' requires a value but none was supplied",
        ));
    }

    #[test]
    //windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn arg_is_correct() -> Result<(), std::io::Error> {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-o", "/tmp/test.txt"]);
        //only check first line
        cmd.assert().stdout(predicate::str::starts_with(
            "Written 3563 bytes to /tmp/test.txt",
        ));
        //delete output file
        fs::remove_file("/tmp/test.txt")
    }
}

pub mod no_color {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--no-color", "123"]);
        cmd.assert().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--no-color");
        //only check first line
        cmd.assert().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod verbosity {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--verbose");
        cmd.assert().stderr(predicate::str::starts_with(
            "error: The argument '--verbose <verbosity>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_info() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--verbose", "info"]);
        //only check first line
        cmd.assert().stderr(predicate::str::contains("INFO"));
    }

    #[test]
    fn arg_debug() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--verbose", "debug"]);
        //only check first line
        cmd.assert().stderr(predicate::str::contains("DEBUG"));
    }

    #[test]
    fn arg_error() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.nonexisting") //this causes a fatal error
            .args(["--verbose", "error"]);
        //only check first line
        cmd.assert().stderr(predicate::str::contains("ERROR"));
    }
}
