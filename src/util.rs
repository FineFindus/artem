use std::env;

///Checks if the terminal supports truecolor mode.
/// Returns false if not.
pub fn supports_truecolor() -> bool {
    match env::var("COLORTERM") {
        Ok(var) => var.contains("truecolor") || var.contains("24bit"),
        Err(_) => false, //not found, true colors are not supported
    }
}

#[cfg(test)]
mod test_color_support {
    use super::*;

    #[test]
    fn true_when_env_is_truecolor() {
        env::set_var("COLORTERM", "truecolor");
        assert_eq!(true, supports_truecolor());
    }

    #[test]
    fn true_when_env_is_24bit() {
        env::set_var("COLORTERM", "24bit");
        assert_eq!(true, supports_truecolor());
    }

    #[test]
    fn false_with_different_env() {
        env::set_var("COLORTERM", "asdas");
        assert_eq!(false, supports_truecolor());
    }
}

//Remap a value from one range to another.
pub fn map_range(from_range: (f64, f64), to_range: (f64, f64), value: f64) -> f64 {
    to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[cfg(test)]
mod test_range {
    use super::*;

    #[test]
    fn remap_values() {
        //remap 2 to 4
        assert_eq!(4f64, map_range((0f64, 10f64), (0f64, 20f64), 2f64));
    }

    #[test]
    fn remap_values_above_range() {
        //remap 21 to 42, since the value will be doubled
        assert_eq!(42f64, map_range((0f64, 10f64), (0f64, 20f64), 21f64));
    }

    #[test]
    fn remap_values_below_range() {
        //remap -1 to -2, since the value will be doubled
        assert_eq!(-2f64, map_range((0f64, 10f64), (0f64, 20f64), -1f64));
    }
}
