use std::fs;

fn load_correct_file() -> String {
    let desired_output = fs::read_to_string("assets/abraham_lincoln.txt").unwrap(); //ignore errors
    desired_output
}

pub mod input {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;

    use crate::load_correct_file;

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
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }

    #[test]
    #[cfg(not(feature = "web_image"))]
    fn url_disabled_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("https://raw.githubusercontent.com/FineFindus/artem/master/examples/abraham_lincoln.jpg");
        //check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .failure()
            .stderr(predicate::str::starts_with("[ERROR] File https://raw.githubusercontent.com/FineFindus/artem/master/examples/abraham_lincoln.jpg does not exist"));
    }

    #[test]
    #[cfg(not(feature = "web_image"))]
    fn help_shows_correct_info_no_url() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("--help");
        cmd.assert().success().stdout(predicate::str::contains(
            //only test beginning, since different formatting would break the rest
            "Paths to the target image. The original image is NOT altered.",
        ));
    }

    #[test]
    fn multiple_input_is_false() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args(["examples/abraham_lincoln.jpg", "examples/non_existing.jpg"]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("does not exist"));
    }

    #[test]
    fn multiple_correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args([
            "examples/abraham_lincoln.jpg",
            "examples/abraham_lincoln.jpg",
        ]);

        let mut ascii_img = String::new();
        //add img twice, since it was given twice as an input
        ascii_img.push_str(&load_correct_file());
        ascii_img.push('\n');
        ascii_img.push_str(&load_correct_file());
        //check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(ascii_img));
    }
}

#[cfg(feature = "web_image")]
pub mod url_input {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn input_does_not_exist() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("https://example.com/no.png");
        cmd.assert().failure().stderr(predicate::str::contains(
            "[ERROR] The image format could not be determined",
        ));
    }

    #[test]
    fn correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        //use example abraham lincoln image from github repo
        cmd.arg("https://raw.githubusercontent.com/FineFindus/artem/master/examples/abraham_lincoln.jpg");
        //check only the first line, the rest is likely to be correct as well
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
            "[ERROR] The image format could not be determined",
        ));
    }

    #[test]
    fn multiple_correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.args([
            "https://raw.githubusercontent.com/FineFindus/artem/master/examples/abraham_lincoln.jpg",
            "https://raw.githubusercontent.com/FineFindus/artem/master/examples/abraham_lincoln.jpg",
        ]);

        let mut ascii_img = String::new();
        //add img twice, since it was given twice as an input
        ascii_img.push_str(&load_correct_file());
        ascii_img.push('\n');
        ascii_img.push_str(&load_correct_file());
        //check only the first line, the rest is likely to be correct as well
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
            //only test beginning, since different formatting would break the rest
            "Paths or URLs to the target image. If the input is an URL, the image is",
        ));
    }
}

pub mod density {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("-c");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--characters <density>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_number() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-c 0.6");
        cmd.assert().success().stdout(predicate::str::starts_with(
            "                      00000  000000000000000000000000000000000000000            ",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-c", "M0123-."]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "MMMMMMMMM00000000000000000000000000000101111111111111111111110011000000000000000",
        ));
    }

    #[test]
    fn arg_preset_0_short_s() {
        for arg in ["short", "s", "0"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().success().stdout(predicate::str::starts_with(
                "###WWWWWW$$$$$999998887776677776666676665555456555555433345566656677788888999$$9",
            ));
        }
    }

    #[test]
    fn arg_preset_1_flat_f() {
        for arg in ["flat", "f", "1"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
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
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().success().stdout(predicate::str::starts_with(
                "W&WMMMM#*oaaaahhhahbbdqwwwwqqwwmwmmmwwZmO0O0Q0Z000000CUJJC0OZmm0Zmqqqdbbddkkaaah",
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
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s string");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_float() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s 0.6");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s -6");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg(format!("-s {}", u32::MAX));
        cmd.assert().failure().stderr(predicate::str::contains(
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
        cmd.assert().failure().stderr(predicate::str::contains(
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
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--height'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").args(["-s", "75"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "WWWNNNNNXXXXXXKXXKKK000OO000OOOOOO0OOOkkkkkOkkkkkkxdxxkkOOOkOO0000KKKKKXXXK",
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
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
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
        cmd.arg("examples/abraham_lincoln.jpg").arg("-w").arg("-h");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--height'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--width");
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
        cmd.arg("examples/abraham_lincoln.jpg").args(["-h", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
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
        cmd.arg("examples/abraham_lincoln.jpg").arg("-h").arg("-w");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--width'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--height");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().success().stdout(predicate::str::starts_with(
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
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--ratio <scale>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "string"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with ratio input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "-6"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '-6' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", f64::MAX.to_string().as_str()]);
        cmd.assert().success().stdout(predicate::str::starts_with(
            "NWWNNNNXXXXXKKKKXXKK00000O000OOOOOOkOOkkxxkxxxOkkkkkkxxdddkkOOOkOOO00KKK00KKXXXX",
        ));
    }

    #[test]
    fn arg_is_zero() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "0"]);
        cmd.assert().success().stdout(predicate::str::starts_with(
            "NNNNXXXXXXKKKKKKKKKKK0OOxddddodooolllllllllooxkkkkkkkkxxxkkkkkkkOOOO0000000KKK0O",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "0.75"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "NWWNNNNNXXXXKKKKXXKKK0000OO00OOOOOOkOOkkxkkkxkOkkkkkkxxddxkkOOOkOO000KKK00KKXXXX",
        ));
    }
}

pub mod flip_x {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--flipX", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--flipX");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "XXXXKKKKKK0000OOkOOOkkxxxxxkkkkkkOkkkkkkOOO0OOOOOOO00OOO000KKKXXKKXXXXXNNNNNNWWW",
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
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--flipY", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--flipY");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "...........................................                                     ",
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
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--outline", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--outline");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "        ................'....'''.''..'..''''',,;;''.',,,,;,'''','',,'''','''....",
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
        cmd.arg("examples/abraham_lincoln.jpg").arg("--hysteresis");
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
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--outline", "--hysteresis", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--outline", "--hys"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "                   ....';''..:;;.;;'.,..,;::;o:ddc,':llcldc:;,:c::lc:;,;c;:,.'.,",
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
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--flipY", "--flipX"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "                                     ..........................................",
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
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--output <output-file>' requires a value but none was supplied",
        ));
    }

    #[test]
    //windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn file_is_ansi() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-o", "/tmp/ascii.ans"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 3644 bytes to /tmp/ascii.ans",
        ));
        //delete output file
        fs::remove_file("/tmp/ascii.ans").unwrap();
    }

    #[test]
    //windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn file_is_html() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-o", "/tmp/ascii.html"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 133620 bytes to /tmp/ascii.html",
        ));
        //delete output file
        fs::remove_file("/tmp/ascii.html").unwrap();
    }

    #[test]
    //windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn file_plain_text() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-o", "/tmp/test.txt"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 3644 bytes to /tmp/test.txt",
        ));
        //delete output file
        fs::remove_file("/tmp/test.txt").unwrap();
    }
}

pub mod invert {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--invert", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--invert");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "        ................'''..'''''''.''',,,,,,',,,,,,;;;;;,,''',''..............",
        ));
    }
}

pub mod background {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--background", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_no_color() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--background", "--no-color"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--background' cannot be used with '--no-color'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--background");
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod no_color {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--no-color", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_conflict_background() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--no-color", "--background"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--no-color' cannot be used with '--background'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--no-color");
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod border {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--border", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "[ERROR] File 123 does not exist\n[ERROR] Artem exited with code: 66\n",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--border");
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

pub mod verbosity {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--verbose");
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--verbose <verbosity>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_info() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--verbose", "info"]);
        //only check first line
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("INFO"));
    }

    #[test]
    fn arg_debug() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--verbose", "debug"]);
        //only check first line
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("DEBUG"));
    }

    #[test]
    fn arg_error() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.nonexisting") //this causes a fatal error
            .args(["--verbose", "error"]);
        //only check first line
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("ERROR"));
    }
}
