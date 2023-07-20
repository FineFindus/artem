use std::num::NonZeroU32;

use crate::util::{self, ResizingDimension};

/// Target for the Ascii conversion.
///
/// This changes of exactly the image is converted and if it supports color.
/// The first boolean determines if the output should be colored, the second if the color is the background color.
///
/// An target might support neither, one or both colors.
///
/// # Examples
///```
/// use artem::config::TargetType;
///
/// assert_eq!(TargetType::Shell(true, false), TargetType::default());
///
///```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    /// Shell target, Supports color and background colors.
    Shell(bool, bool),
    /// Special Ansi/ans file that will always have colors enabled. Can also have background colors.
    AnsiFile(bool),
    /// HTML target, Supports color and background colors.
    HtmlFile(bool, bool),
    /// Every other file, does not support either colored outputs.
    File,
}

impl Default for TargetType {
    /// Default [`TargetType`]
    ///
    /// The default [`TargetType`] is the shell with colors enabled.
    /// Background colors are disabled by default.
    ///
    /// # Examples
    /// ```
    /// use artem::config::TargetType;
    ///
    /// assert_eq!(TargetType::Shell(true, false), TargetType::default());
    /// ```
    fn default() -> TargetType {
        TargetType::Shell(true, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(TargetType::Shell(true, false), TargetType::default());
    }
}

///Config for the conversion of the image to the ascii image.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub characters: String,
    pub scale: f32,
    pub target_size: u32,
    pub invert: bool,
    pub border: bool,
    pub dimension: ResizingDimension,
    pub transform_x: bool,
    pub transform_y: bool,
    pub center_x: bool,
    pub center_y: bool,
    pub outline: bool,
    pub hysteresis: bool,
    pub target: TargetType,
}

impl Config {
    /// Create [`ConfigBuilder`] with default properties.
    ///
    /// # Examples
    /// ```
    /// use artem::config::Config;
    ///
    /// let default = Config::builder();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
            scale: 0.42f32,
            target_size: 80,
            invert: Default::default(),
            border: Default::default(),
            dimension: Default::default(),
            transform_x: Default::default(),
            transform_y: Default::default(),
            center_x: Default::default(),
            center_y: Default::default(),
            outline: Default::default(),
            hysteresis: Default::default(),
            target: Default::default(),
        }
    }
}

#[cfg(test)]
mod test_option {
    use super::*;
    #[test]
    fn builder_default() {
        assert_eq!(
            ConfigBuilder {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            Config::builder()
        );
    }
}

///A builder to create a [`Config`] struct.
#[derive(PartialEq, Debug)]
pub struct ConfigBuilder {
    characters: String,
    scale: f32,
    target_size: u32,
    invert: bool,
    border: bool,
    dimension: ResizingDimension,
    transform_x: bool,
    transform_y: bool,
    center_x: bool,
    center_y: bool,
    outline: bool,
    hysteresis: bool,
    target: TargetType,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            //these have to be set to custom defaults for the program to work
            characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
            scale: 0.42f32,
            target_size: 80,
            invert: Default::default(),
            border: Default::default(),
            dimension: Default::default(),
            transform_x: Default::default(),
            transform_y: Default::default(),
            center_x: Default::default(),
            center_y: Default::default(),
            outline: Default::default(),
            hysteresis: Default::default(),
            target: Default::default(),
        }
    }
}

/// Generate a builder property.
///
/// This macro generates a function, which sets the specified property of the Builder.
/// It does NOT check the value passed in, so for example it would be possible to pass in an empty string, which could lead to
/// errors later this. This should be done before setting the value.
///
/// # Examples
///
/// ``` compile_fail, just an internal example code
/// property!{
/// ///Example doc
/// ///This generates a name setter function.
/// => name, String
/// }
/// ```
/// ## Generated function
/// ``` compile_fail, just an internal example code
/// ///Example doc
/// ///This generates a name setter function.
/// pub fn name<'a>(&'a mut self, name: String) -> &'a Self {
///     self.name = name;
///     self
/// }
/// ```
macro_rules! property {
    ($(#[$attr:meta])* => $field:ident, $field_type:ty) => {
        $(#[$attr])*
        pub fn $field(&mut self, $field: $field_type) -> &mut Self {
            self.$field = $field;
            self
        }
    };
    ($(#[$attr:meta])* => $field:ident, $field_type:ty, $func:ident) => {
        $(#[$attr])*
        pub fn $field(&mut self, $field: $field_type) -> &mut Self {
            self.$field = $field.$func();
            self
        }
    };
}

impl ConfigBuilder {
    ///Create a new ConfigBuilder.
    ///
    /// If an option is not specified, the rust default value will be used, unless specified otherwise.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// ```
    pub fn new() -> ConfigBuilder {
        Config::builder()
    }

    ///Set the characters.
    ///
    /// The characters will determine how 'visible'/light/dark a character will be perceived.
    ///
    /// # Errors
    /// When the given characters are empty, the characters will not be changed.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.characters("Mkl. ".to_string());
    /// ```
    #[allow(clippy::needless_lifetimes)] //disable this, as the life is needed for the builder
    pub fn characters<'a>(&'a mut self, characters: String) -> &'a Self {
        if !characters.is_empty() {
            self.characters = characters;
        }
        self
    }

    property! {
    /// Set the scale.
    ///
    /// Used to change the ratio between width and height of an character.
    /// Since a char is a bit higher than wide, the scale should compensate for that.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.scale(0.42f32);
    /// ```
       => scale, f32
    }
    // pub fn scale(&'a mut self, scale: f32) -> &'a mut Self {
    //     self.scale = scale;
    //     self
    // }

    property! {
    /// Set the target_size.
    ///
    /// This is the number of characters that the resulting ascii image will be high/wide.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    /// use core::num::NonZeroU32;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.target_size(NonZeroU32::new(80).unwrap());
    /// ```
    => target_size, NonZeroU32, get
    }

    property! {
    ///Invert to density map/characters.
    ///
    /// This inverts the mapping from light to dark characters. It can be useful when
    /// the image has a dark background. It defaults to false.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.invert(true);
    /// ```
    => invert, bool
    // pub fn invert(mut self, invert: bool) -> Self {
    //     self.invert = invert;
    //     self
    }

    property! {
    ///Enable a border surrounding the image.
    ///
    /// The border will take reduced the space of the ascii image, since it will still
    /// use the same target size.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.border(true);
    /// ```
    => border, bool
    }

    property! {
    /// Set which dimension should be scaled first.
    ///
    /// See [`ResizingDimension`] for more information.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    /// use artem::util::ResizingDimension;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.dimension(ResizingDimension::Height);
    /// ```
    => dimension, util::ResizingDimension
    }

    property! {
    ///Flip the image along the X-axis
    ///
    /// This will flip the image horizontally by reversing reading of the horizontal part of the image.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.transform_x(true);
    /// ```
    => transform_x, bool
    }

    property! {
    ///Flip the image along the Y-axis
    ///
    /// This will flip the image vertically by reversing reading of the vertical part of the image.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.transform_y(true);
    /// ```
    => transform_y, bool
    }

    property! {
    /// Center the image horizontally in the terminal
    ///
    /// It will center the image by adding spaces in front of the text.
    /// Since the terminal might have an uneven width, and only monospace chars are allowed,
    /// the spacing might not always be accurate.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.center_x(true);
    /// ```
    => center_x, bool
    }

    property! {
    /// Center the image vertically in the terminal
    ///
    /// It will center the image by adding new lines  above and below the text.
    /// Since the terminal might have an uneven height, and only monospace chars are allowed,
    /// the spacing might not always be accurate.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.center_y(true);
    /// ```
    => center_y, bool
    }

    property! {
    ///Convert the image to it's outline
    ///
    /// This will use gaussian blur and sobel operators to only it's outline,
    /// it might not produce the best result on all images.
    /// Caution, this will take some additional time.
    /// Defaults to false.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.outline(true);
    /// ```
    => outline, bool
    }

    property! {
    /// When converting the image to an outline, also use double threshold and hysteresis
    ///
    /// It will remove small imperfection in the outline for the cost of smaller/thinner lines, which can make
    /// the resulting ascii looking less visible, which might not be desired.
    ///
    /// It will only be used when outlining is set to true.
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.hysteresis(true);
    /// ```
    => hysteresis, bool
    }

    property! {
    ///Set the target type
    ///
    /// This will effect the output, for example if the [`TargetType`] is set to html,
    /// the output will be the content of a Html-file.
    ///
    /// See [`TargetType`] for more information. It defaults to the shell as the target.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    /// use artem::config::TargetType;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// //use color, but don't color the background
    /// builder.target(TargetType::HtmlFile(true, false));
    /// ```
        => target,  TargetType
    }

    ///Build the [`Config`] struct.
    ///
    /// This returns a [`Config`], which can than be used for the image conversion using [`super::convert()`].
    /// If values are not explicitly specified, the default values will be used.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// let options = builder.build();
    /// ```
    pub fn build(&self) -> Config {
        Config {
            characters: self.characters.to_owned(),
            scale: self.scale,
            target_size: self.target_size,
            invert: self.invert,
            border: self.border,
            dimension: self.dimension,
            transform_x: self.transform_x,
            transform_y: self.transform_y,
            center_x: self.center_x,
            center_y: self.center_y,
            outline: self.outline,
            hysteresis: self.hysteresis,
            target: self.target,
        }
    }
}

#[cfg(test)]
mod test_conversion_configuration_builder {
    use super::*;
    #[test]
    fn build_default() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().build()
        );
    }

    #[test]
    fn change_characters() {
        assert_eq!(
            Config {
                characters: r#"characters"#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new()
                .characters("characters".to_string())
                .build()
        );
    }

    #[test]
    fn change_scale() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 3.14f32, //change attribute
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().scale(3.14f32).build()
        );
    }

    #[test]
    fn change_target_size() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 314, //change attribute
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new()
                .target_size(NonZeroU32::new(314).unwrap())
                .build()
        );
    }

    #[test]
    fn change_invert() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: true, //change attribute
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().invert(true).build()
        );
    }

    #[test]
    fn change_border() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: true, //change attribute
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().border(true).build()
        );
    }

    #[test]
    fn change_dimension() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Height, //change attribute
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new()
                .dimension(util::ResizingDimension::Height)
                .build()
        );
    }

    #[test]
    fn change_transform_x() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: true, //change attribute
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().transform_x(true).build()
        );
    }

    #[test]
    fn change_transform_y() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: true, //change attribute
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().transform_y(true).build()
        );
    }

    #[test]
    fn change_center_x() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: true, //change attribute
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().center_x(true).build()
        );
    }

    #[test]
    fn change_center_y() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: true, //change attribute
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().center_y(true).build()
        );
    }

    #[test]
    fn change_outline() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: true, //change attribute
                hysteresis: false,
                target: TargetType::default(),
            },
            ConfigBuilder::new().outline(true).build()
        );
    }

    #[test]
    fn change_hysteresis() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: true, //change attribute
                target: TargetType::default(),
            },
            ConfigBuilder::new().hysteresis(true).build()
        );
    }

    #[test]
    fn change_file_type() {
        assert_eq!(
            Config {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                center_x: false,
                center_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::AnsiFile(false), //change attribute
            },
            ConfigBuilder::new()
                .target(TargetType::AnsiFile(false))
                .build()
        );
    }
}
