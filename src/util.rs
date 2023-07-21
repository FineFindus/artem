use std::env;

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
