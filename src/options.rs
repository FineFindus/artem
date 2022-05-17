use std::num::NonZeroU32;

use crate::util::{self, ResizingDimension};

/// Target for the Ascii conversion.
///
/// This changes of exactly the image is converted and if it supports color.
/// The first boolean determines if the output should be colored, the second if the color is the background color.
///
/// An target might support none, one or both colors.
///
/// # Examples
///```
/// use artem::options::TargetType;
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
    /// Shell target, Supports color and background colors.
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
    /// use artem::options::TargetType;
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

///Configuration for the conversion of the image to the ascii image.
#[derive(Debug, PartialEq)]
pub struct Option {
    pub characters: String,
    pub scale: f32,
    pub target_size: u32,
    pub invert: bool,
    pub border: bool,
    pub dimension: ResizingDimension,
    pub transform_x: bool,
    pub transform_y: bool,
    pub outline: bool,
    pub hysteresis: bool,
    pub target: TargetType,
}

impl Option {
    /// Create [`OptionBuilder`] with default properties.
    ///
    /// # Examples
    /// ```
    /// use artem::options::Option;
    ///
    /// let default = Option::builder();
    /// ```
    pub fn builder() -> OptionBuilder {
        OptionBuilder::default()
    }
}

#[cfg(test)]
mod test_option {
    use super::*;
    #[test]
    fn builder_default() {
        assert_eq!(
            OptionBuilder {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            Option::builder()
        );
    }
}

///A builder to create a [`Option`] struct.
#[derive(PartialEq, Debug)]
pub struct OptionBuilder {
    characters: String,
    scale: f32,
    target_size: u32,
    invert: bool,
    border: bool,
    dimension: ResizingDimension,
    transform_x: bool,
    transform_y: bool,
    outline: bool,
    hysteresis: bool,
    target: TargetType,
}

impl Default for OptionBuilder {
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
        pub fn $field<'a>(&'a mut self, $field: $field_type) -> &'a mut Self {
            self.$field = $field;
            self
        }
    };
    ($(#[$attr:meta])* => $field:ident, $field_type:ty, $func:ident) => {
        $(#[$attr])*
        pub fn $field<'a>(&'a mut self, $field: $field_type) -> &'a mut Self {
            self.$field = $field.$func();
            self
        }
    };
}

// impl Default  {

// }

impl OptionBuilder {
    ///Create a new OptionBuilder.
    ///
    /// If an option is not specified, the rust default value will be used, unless specified otherwise.
    ///
    /// # Examples
    /// ```
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
    /// ```
    pub fn new() -> OptionBuilder {
        Option::builder()
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// This is the number of characters that the resulting ascii image will be heigh/wide.
    ///
    /// # Examples
    /// ```
    /// use artem::options::OptionBuilder;
    /// use core::num::NonZeroU32;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    /// use artem::util::ResizingDimension;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
    /// builder.transform_y(true);
    /// ```
    => transform_y, bool
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
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
    /// use artem::options::OptionBuilder;
    /// use artem::options::TargetType;
    ///
    /// let mut builder = OptionBuilder::new();
    /// //use color, but don't color the background
    /// builder.target(TargetType::HtmlFile(true, false));
    /// ```
        => target,  TargetType
    }

    ///Build the [`Option`] struct.
    ///
    /// This returns a [`Option`], which can than be used for the image conversion using [`convert()`].
    /// If values are not explicitly specified, the default values will be used.
    ///
    /// # Examples
    /// ```
    /// use artem::options::OptionBuilder;
    ///
    /// let mut builder = OptionBuilder::new();
    /// let options = builder.build();
    /// ```
    pub fn build(&self) -> Option {
        Option {
            characters: self.characters.to_owned(),
            scale: self.scale,
            target_size: self.target_size,
            invert: self.invert,
            border: self.border,
            dimension: self.dimension,
            transform_x: self.transform_x,
            transform_y: self.transform_y,
            outline: self.outline,
            hysteresis: self.hysteresis,
            target: self.target,
        }
    }
}

#[cfg(test)]
mod test_conversion_option_builder {
    use super::*;
    #[test]
    fn build_default() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().build()
        );
    }

    #[test]
    fn change_characters() {
        assert_eq!(
            Option {
                characters: r#"characters"#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new()
                .characters("characters".to_string())
                .build()
        );
    }

    #[test]
    fn change_scale() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 3.14f32, //change attribute
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().scale(3.14f32).build()
        );
    }

    #[test]
    fn change_target_size() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 314, //change attribute
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new()
                .target_size(NonZeroU32::new(314).unwrap())
                .build()
        );
    }

    #[test]
    fn change_invert() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: true, //change attribute
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().invert(true).build()
        );
    }

    #[test]
    fn change_border() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: true, //change attribute
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().border(true).build()
        );
    }

    #[test]
    fn change_dimension() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Height, //change attribute
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new()
                .dimension(util::ResizingDimension::Height)
                .build()
        );
    }

    #[test]
    fn change_transform_x() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: true, //change attribute
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().transform_x(true).build()
        );
    }

    #[test]
    fn change_transform_y() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: true, //change attribute
                outline: false,
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().transform_y(true).build()
        );
    }

    #[test]
    fn change_outline() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: true, //change attribute
                hysteresis: false,
                target: TargetType::default(),
            },
            OptionBuilder::new().outline(true).build()
        );
    }

    #[test]
    fn change_hysteresis() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: true, //change attribute
                target: TargetType::default(),
            },
            OptionBuilder::new().hysteresis(true).build()
        );
    }

    #[test]
    fn change_file_type() {
        assert_eq!(
            Option {
                characters: r#"MWNXK0Okxdolc:;,'...   "#.to_string(),
                scale: 0.42f32,
                target_size: 80,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: TargetType::AnsiFile(false), //change attribute
            },
            OptionBuilder::new()
                .target(TargetType::AnsiFile(false))
                .build()
        );
    }
}
