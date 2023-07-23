use std::fs;
///! Utilities and common function between tests.
///! It includes functions to help loading expected results to compare against.

/// Load the correct files.
///
/// Loads a string containing the correct and expected result of the command output.
/// The returned String does not have color.
pub fn load_correct_file() -> String {
    //ignore errors
    fs::read_to_string("assets/standard_test_img/standard_test_img.txt").unwrap()
}
