use std::num::NonZeroU32;

///Preferred image resize direction
///
///This changes which dimensions should be used when resizing the image.
///For example, to fully use one dimension (e.g. width), the height can not be scaled
///up as well, since it already would be larger than the maximum terminal height.
///By default width will be used.
///
/// # Examples
/// ```
/// use artem::config::ResizingDimension;
///
/// assert_eq!(ResizingDimension::Width, ResizingDimension::default());
/// ```
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResizingDimension {
    #[default]
    Width,
    Height,
}

impl ResizingDimension {
    /// Calculate image dimension related values.
    ///
    /// This calculates the number of columns, rows, and the tile dimensions (tile_width, tile_height) for these
    /// values based on a target_size. It returns them as a tuple, the elements are in the previously named order.
    /// The dimension property can be used to change what dimension will be scaled. Since terminal character are a bit higher the wide,
    /// Width and Height of the output needs to be based on either one, so the other can be calculated.
    ///
    /// # Examples
    /// ```
    /// use artem::config::{ResizingDimension};
    ///
    /// assert_eq!(
    /// (100, 46, 5, 11),
    /// //image with a size of 512x512, split into 100 columns with no border
    /// ResizingDimension::calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Width));
    /// ```
    pub fn calculate_dimensions(
        target_size: u32,
        height: u32,
        width: u32,
        scale: f32,
        border: bool,
        dimension: ResizingDimension,
    ) -> (u32, u32, u32, u32) {
        match dimension {
            ResizingDimension::Width => {
                //calculate dimensions based on columns
                let mut columns = if width > target_size {
                    target_size
                } else {
                    width
                };

                if border {
                    //remove a bit of space for the border
                    columns = columns.saturating_sub(2).max(1); //should be at last 1
                }

                //calculate tiles
                let tile_width = width / columns;
                let tile_height = (tile_width as f32 / scale).floor() as u32;

                let rows = height / tile_height;

                //.max(1) is used to ensure that the values are at least 1
                //a value of 0 could cause an error (but not crash) later on
                (columns.max(1), rows.max(1), tile_width, tile_height)
            }

            ResizingDimension::Height => {
                let mut rows = if height > target_size {
                    // minus 1, since the user input line is included
                    target_size - 1
                } else {
                    height
                };

                //calculate tiles
                let tile_height = height / rows;
                let tile_width = (tile_height as f32 * scale).ceil() as u32;

                let mut columns = width / tile_width;

                if border {
                    //remove a bit of space for the border
                    columns = columns.saturating_sub(2);
                    rows = rows.saturating_sub(2);
                }

                //.max(1) is used to ensure that the values are at least 1
                //a value of 0 could cause an error (but not crash) later on
                (columns.max(1), rows.max(1), tile_width, tile_height)
            }
        }
    }
}

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
/// assert_eq!(TargetType::Shell, TargetType::default());
///```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetType {
    /// Stdout
    ///
    /// Supports both color and background colors.
    #[default]
    Shell,
    /// Special Ansi/ans file that will always have colors enabled.
    /// Can also have background colors.
    AnsiFile,
    /// HTML target
    ///
    /// Supports color and background colors.
    HtmlFile,
    /// Every other file
    ///
    /// Does not support either colored outputs.
    File,
    /// SVG
    ///
    /// Supports both color and background colors.
    /// The resulting output file may have visual glitches.
    Svg,
}

impl TargetType {
    /// Returns whether the output supports colored output.
    pub(crate) fn supports_color(&self) -> bool {
        //all targets, except for raw file support colored output
        self != &TargetType::File
    }

    /// Returns whether the output supports background colors
    pub(crate) fn supports_background_colors(&self) -> bool {
        match self {
            TargetType::Shell | TargetType::HtmlFile | TargetType::Svg => true,
            TargetType::AnsiFile | TargetType::File => false,
        }
    }
}

///Config for the conversion of the image to the ascii image.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub characters: String,
    pub scale: f32,
    pub target_size: u32,
    pub color: bool,
    pub background_color: bool,
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

    /// Returns whether the output should be colored.
    ///
    /// This depends on both if colored output is enabled and the output target supports colored
    /// output.
    pub(crate) fn color(&self) -> bool {
        self.color && self.target.supports_color()
    }

    /// Returns whether the output should be colored using background colors.
    ///
    /// This depends on both if background color is enabled and the output target supports
    /// background colored output.
    pub(crate) fn background_color(&self) -> bool {
        self.background_color && self.target.supports_background_colors()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
            scale: 0.42f32,
            target_size: 80,
            color: true,
            background_color: false,
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
                color: true,
                background_color: false,
                invert: false,
                border: false,
                dimension: ResizingDimension::Width,
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
    color: bool,
    background_color: bool,
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
            color: true,
            background_color: false,
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

    /// Set the characters.
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
    pub fn characters(&mut self, characters: String) -> &mut Self {
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
    /// Render the output in color.
    ///
    /// If the chosen [`TargetType`] does not support color, it is ignored.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.color(true);
    /// ```
    => color, bool
    }

    property! {
    /// Render the background of the output in color.
    ///
    /// If the chosen [`TargetType`] does not support background colors, it is ignored.
    ///
    /// # Examples
    /// ```
    /// use artem::config::ConfigBuilder;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.color(true);
    /// ```
    => background_color, bool
    }

    property! {
    /// Invert to density map/characters.
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
    }

    property! {
    /// Enable a border surrounding the image.
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
    /// use artem::config::ResizingDimension;
    ///
    /// let mut builder = ConfigBuilder::new();
    /// builder.dimension(ResizingDimension::Height);
    /// ```
    => dimension, ResizingDimension
    }

    property! {
    /// Flip the image along the X-axis
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
    /// Flip the image along the Y-axis
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
    /// Convert the image to it's outline
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
    /// Set the target type
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
    /// builder.target(TargetType::HtmlFile);
    /// ```
        => target,  TargetType
    }

    /// Build the [`Config`] struct.
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
            color: self.color,
            background_color: self.background_color,
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
                color: true,
                background_color: false,
                invert: false,
                border: false,
                dimension: ResizingDimension::Width,
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
}

#[cfg(test)]
mod test_calculate_dimensions {
    use super::*;

    #[test]
    fn calculate_dimensions_width() {
        assert_eq!(
            (100, 46, 5, 11),
            ResizingDimension::calculate_dimensions(
                100,
                512,
                512,
                0.42,
                false,
                ResizingDimension::Width
            )
        );
    }

    #[test]
    fn calculate_dimensions_width_119() {
        assert_eq!(
            (119, 56, 4, 9),
            ResizingDimension::calculate_dimensions(
                119,
                512,
                512,
                0.42,
                false,
                ResizingDimension::Width
            )
        );
    }

    #[test]
    fn calculate_dimensions_height() {
        assert_eq!(
            (170, 99, 3, 5),
            ResizingDimension::calculate_dimensions(
                100,
                512,
                512,
                0.42,
                false,
                ResizingDimension::Height
            )
        );
    }

    #[test]
    fn calculate_dimensions_height_1x1_img() {
        assert_eq!(
            (1, 1, 1, 1),
            ResizingDimension::calculate_dimensions(
                100,
                1,
                1,
                0.42,
                false,
                ResizingDimension::Height
            )
        );
    }

    #[test]
    fn calculate_dimensions_width_1x1_img() {
        assert_eq!(
            (1, 1, 1, 2),
            ResizingDimension::calculate_dimensions(
                100,
                1,
                1,
                0.42,
                false,
                ResizingDimension::Width
            )
        );
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_height_zero() {
        ResizingDimension::calculate_dimensions(
            0,
            512,
            512,
            0.42,
            false,
            ResizingDimension::Height,
        );
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_width_zero() {
        ResizingDimension::calculate_dimensions(0, 512, 512, 0.42, false, ResizingDimension::Width);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_width_zero() {
        ResizingDimension::calculate_dimensions(100, 512, 0, 0.42, false, ResizingDimension::Width);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_height_zero() {
        ResizingDimension::calculate_dimensions(
            100,
            0,
            512,
            0.42,
            false,
            ResizingDimension::Height,
        );
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_width_height_zero() {
        ResizingDimension::calculate_dimensions(100, 0, 0, 0.42, false, ResizingDimension::Height);
    }

    #[test]
    fn calculate_dimensions_scale_zero() {
        assert_eq!(
            (100, 1, 5, 4294967295),
            ResizingDimension::calculate_dimensions(
                100,
                512,
                512,
                0f32,
                false,
                ResizingDimension::Width
            )
        );
    }

    #[test]
    fn calculate_border_smaller_columns() {
        assert_eq!(
            (98, 1, 5, 4294967295),
            ResizingDimension::calculate_dimensions(
                100,
                512,
                512,
                0f32,
                true,
                ResizingDimension::Width
            )
        );
    }
}
