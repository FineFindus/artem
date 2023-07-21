use std::{env, process};

use once_cell::sync::Lazy;

///Returns if the terminal supports truecolor mode.
///
/// It checks the `COLORTERM` environment variable,
/// if it is either set to
/// `truecolor` or `24bit` true is returned.
///
/// In all other cases false will be returned.
///
/// # Examples
/// ```
/// use artem::util::SUPPORTS_TRUECOLOR;
/// # use std::env;
///
/// # env::set_var("COLORTERM", "truecolor");
/// //only true when run in a shell that supports true color
/// let color_support = *SUPPORTS_TRUECOLOR;
/// assert!(color_support);
/// ```
pub static SUPPORTS_TRUECOLOR: Lazy<bool> = Lazy::new(|| {
    env::var("COLORTERM").is_ok_and(|value| value.contains("truecolor") || value.contains("24bit"))
});

#[cfg(test)]
mod test_color_support {
    use super::*;

    #[test]
    #[ignore = "Requires truecolor support"]
    fn true_when_env_is_truecolor() {
        env::set_var("COLORTERM", "truecolor");
        assert!(*SUPPORTS_TRUECOLOR);
    }

    #[test]
    #[ignore = "Requires truecolor support"]
    fn true_when_env_is_24bit() {
        env::set_var("COLORTERM", "24bit");
        assert!(*SUPPORTS_TRUECOLOR);
    }

    #[test]
    fn false_with_different_env_false() {
        env::set_var("COLORTERM", "false");
        assert!(!*SUPPORTS_TRUECOLOR);
    }

    #[test]
    fn false_with_different_env() {
        env::set_var("COLORTERM", "kjasdlkdjaskd");
        assert!(!*SUPPORTS_TRUECOLOR);
    }
}

///Function for fatal errors.
///
///A fatal error is an error, from which the program can no recover, meaning the only option left is to print
/// an error message letting the user know what went wrong. For example if a non-existing file was passed in,
/// this program can not work correctly and should print an error message and exit.
///
/// This function will print the passed in error message as well as a exit message, then it will exit the program with the exit code.
/// If non is specified, it will use exit code 1 by default.
/// A list of exit code can be found here: <https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html>
///
/// # Examples
/// ```no_run
/// use std::fs::File;
/// use artem::util::fatal_error;
///
/// let f = File::open("hello.txt");
/// let f = match f {
///     Ok(file) => file,
///     Err(error) => fatal_error(&error.to_string(), Some(66)),
/// };
/// ```
pub fn fatal_error(message: &str, code: Option<i32>) -> ! {
    //This function never returns, since it always exit the program
    log::error!("{}", message);
    log::error!("Artem exited with code: {}", code.unwrap_or(1));
    process::exit(code.unwrap_or(1));
}
