///Returns the top part of the output html file.
///
/// This contains the html elements needed for a correct html file.
/// The title will be set to `Artem Ascii Image`, whilst the will be set to `Courier` ( a monospace font)
/// It will also have the pre tag for correct spacing/line breaking
///
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
/// use artem::target::html;
///
/// let string = String::new();
/// string.push_str(&html_top())
/// ```
pub fn html_top() -> String {
    r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Artem Ascii Image</title>
        <style>* {font-family: Courier;}</style>
    </head>
    
    <body>
        <pre>"#
        .to_string()
}

#[cfg(test)]
mod test_push_html_top {
    use super::*;
    #[test]
    fn push_top_html_returns_correct_string() {
        assert_eq!(
            r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Artem Ascii Image</title>
        <style>* {font-family: Courier;}</style>
    </head>
    
    <body>
        <pre>"#,
            html_top()
        )
    }
}

///Returns the bottom part of the output html file.
///
/// The matching closing tags fro [`html_top`]. It will close
/// the pres, body and html tag.
///
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
/// use artem::target::html;
///
/// let string = String::new();
/// string.push_str(&html_top())
/// string.push_str(&html_bottom())
/// ```
pub fn html_bottom() -> String {
    "\n</pre></body></html>".to_string()
}

#[cfg(test)]
mod test_push_html_bottom {
    use super::*;

    #[test]
    fn push_bottom_html_returns_correct_string() {
        assert_eq!("\n</pre></body></html>", html_bottom())
    }
}

/// Returns an html string representation of the given char with optional background color support.
///
/// Creates an <span> element with style attribute, which sets the (background) color to the
/// given rgb inputs.
/// Technically the span can have more than a single char, but the complexity needed for a system to group
/// characters with the same color would be unnecessary and out of scope.
///
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
/// println!("{}", get_html(100, 100, 100, 'x', false));
/// ```
pub fn colored_char(red: u8, green: u8, blue: u8, char: char, background_color: bool) -> String {
    if background_color {
        format!(
            "<span style=\"background-color: #{:02X?}{:02X?}{:02X?}\">{}</span>",
            red, green, blue, char
        )
    } else {
        if char.is_whitespace() {
            //white spaces don't have a visible foreground color,
            //it saves space when not  having an entire useless span tag
            return String::from(char);
        } else {
            format!(
                "<span style=\"color: #{:02X?}{:02X?}{:02X?}\">{}</span>",
                red, green, blue, char
            )
        }
    }
}

#[cfg(test)]
mod test_html_string {
    use super::*;

    #[test]
    fn whitespace_no_tag() {
        assert_eq!(" ", colored_char(0, 0, 0, ' ', false))
    }

    #[test]
    fn black_no_background() {
        assert_eq!(
            "<span style=\"color: #000000\">x</span>",
            colored_char(0, 0, 0, 'x', false)
        )
    }

    #[test]
    fn black_with_background() {
        assert_eq!(
            "<span style=\"background-color: #000000\">x</span>",
            colored_char(0, 0, 0, 'x', true)
        )
    }

    #[test]
    fn rust_color_no_background() {
        assert_eq!(
            "<span style=\"color: #9A5536\">x</span>",
            colored_char(154, 85, 54, 'x', false)
        )
    }

    #[test]
    fn rust_color_with_background() {
        assert_eq!(
            "<span style=\"background-color: #9A5536\">x</span>",
            colored_char(154, 85, 54, 'x', true)
        )
    }
}
