use std::num::NonZeroU32;

use crate::util::{self, ResizingDimension};

/// Target for the Ascii conversion.
///
/// This changes of exactly the image is converted and if it supports color.
/// The first boolean determines if the output should be colored, the second if the color is rhe background color.
///
/// An target might support none, one or both colors.
///
/// # Examples
///```
/// assert_eq!(ConversionTargetType::Shell(true, false), ConversionTargetType::default());
///
///```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionTargetType {
    ///Shell target, Supports color and background colors.
    Shell(bool, bool),
    ///Special Ansi/ans file that will always have colors enabled. Can also have background colors.
    AnsiFile(bool),
    ///Shell target, Supports color and background colors.
    HtmlFile(bool, bool),
    ///Every other file, does not support either colored outputs.
    File,
}

impl Default for ConversionTargetType {
    ///Default ConversionTargetType
    ///
    /// The default [ConversionTargetType] is the shell with colors enabled.
    /// Background colors are disabled by default.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(ConversionTargetType::Shell(true, false), ConversionTargetType::default());
    /// ```
    fn default() -> ConversionTargetType {
        ConversionTargetType::Shell(true, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            ConversionTargetType::Shell(true, false),
            ConversionTargetType::default()
        );
    }
}

///Configuration for the conversion of the image to the ascii image.
#[derive(Debug, PartialEq)]
pub struct ConversionOption {
    pub density: String,
    pub threads: u32,
    pub scale: f64,
    pub target_size: u32,
    pub invert: bool,
    pub border: bool,
    pub dimension: ResizingDimension,
    pub transform_x: bool,
    pub transform_y: bool,
    pub outline: bool,
    pub hysteresis: bool,
    pub target: ConversionTargetType,
}

impl ConversionOption {
    /// Create [ConversionOptionBuilder] with default properties.
    ///
    /// # Examples
    /// ```
    /// let default = ConversionOption::builder();
    /// ```
    pub fn builder() -> ConversionOptionBuilder {
        ConversionOptionBuilder::default()
    }
}

#[cfg(test)]
mod test_conversion_option {
    use super::*;
    #[test]
    fn builder_default() {
        assert_eq!(
            ConversionOptionBuilder {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOption::builder()
        );
    }
}

///A builder to create a [ConversionOption] struct.
#[derive(Default, PartialEq, Debug)]
pub struct ConversionOptionBuilder {
    density: String,
    threads: u32,
    scale: f64,
    target_size: u32,
    invert: bool,
    border: bool,
    dimension: ResizingDimension,
    transform_x: bool,
    transform_y: bool,
    outline: bool,
    hysteresis: bool,
    target: ConversionTargetType,
}

/// Generate a builder property.
///
/// This macro generates a function, which sets the specified property of the Builder.
/// It does NOT check the value passed in, so for example it would be possible to pass in an empty string, which could lead to
/// errors later this. This should be done before setting the value.
///
/// # Examples
///
/// ```
/// property!{
/// ///Example doc
/// ///This generates a name setter function.
/// => name, String
/// }
/// ```
/// ## Generated function
/// ```
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

impl ConversionOptionBuilder {
    ///Create a new ConversionOptionBuilder.
    ///
    /// If an option is not specified, the rust default value will be used, unless specified otherwise.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// ```
    pub fn new() -> ConversionOptionBuilder {
        ConversionOption::builder()
    }

    ///Set the density.
    ///
    /// The density will determine how 'visible'/light/dark a character will be perceived.
    ///
    /// # Errors
    /// When the given density is empty, the density will not be changed.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.density("Mkl. ");
    /// ```
    #[allow(clippy::needless_lifetimes)] //disable this, as the life is needed for the builder
    pub fn density<'a>(&'a mut self, density: String) -> &'a Self {
        if density.is_empty() {
            return self;
        }
        self.density = density;
        self
    }

    property! {
    /// Set the number of threads used to convert the image.
    ///
    /// Should be at least 1 and not more then the number of tiles that will be converted.
    /// More threads can lead to greater performance, but too many can also reduced performance,
    /// since each thread has an creation cost.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.thread(4);
    /// ```
    => threads, NonZeroU32, get
    }

    property! {
    /// Set the scale.
    ///
    /// Used to change the ratio between width and height of an character.
    /// Since a char is a bit higher than wide, the scale should compensate for that.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.scale(0.42f64);
    /// ```
       => scale, f64
    }
    // pub fn scale(&'a mut self, scale: f64) -> &'a mut Self {
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
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.target_size(80);
    /// ```
    => target_size, NonZeroU32, get
    }

    property! {
    ///Invert to density map/character.
    ///
    /// This inverts the mapping from light to dark characters. It can be useful when
    /// the image has a dark background. It defaults to false.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
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
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.border(true);
    /// ```
    => border, bool
    // pub fn border(mut self, border: bool) -> Self {
    //     self.border = border;
    //     self
    }

    property! {
    /// Set which dimension should be scaled first.
    ///
    /// See [ResizingDimension] for more information.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.dimension(util::ResizingDimension:.Height);
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
    /// let mut builder = ConversionOptionBuilder::new();
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
    /// let mut builder = ConversionOptionBuilder::new();
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
    /// let mut builder = ConversionOptionBuilder::new();
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
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder.hysteresis(true);
    /// ```
    => hysteresis, bool
    }

    property! {
        ///Set the target type
        ///
        /// This will effect the output, for example if the [ConversionTargetType] is set to html,
        /// the output will be the content of a Html-file.
        ///
        /// See [ConversionTargetType] for more information. It defaults to the shell as the target.
        ///
        /// # Examples
        /// ```
        /// let mut builder = ConversionOptionBuilder::new();
        /// builder.file_type(ConversionFileType::Html);
        /// ```
        => target,  ConversionTargetType
    }

    ///Build the ConversionOptions struct.
    ///
    /// This returns a [ConversionOption], which can than be used for the image conversion.
    /// If values are not explicitly specified, the default values will be used.
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// let options = builder.build();
    /// ```
    pub fn build(&self) -> ConversionOption {
        ConversionOption {
            density: self.density.to_owned(),
            threads: self.threads,
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
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().build()
        );
    }

    #[test]
    fn change_density() {
        assert_eq!(
            ConversionOption {
                density: "density".to_string(), //change attribute
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new()
                .density("density".to_string())
                .build()
        );
    }

    #[test]
    fn change_thread_count() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 314, //change attribute
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new()
                .threads(NonZeroU32::new(314).unwrap())
                .build()
        );
    }

    #[test]
    fn change_scale() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 3.14f64, //change attribute
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().scale(3.14f64).build()
        );
    }

    #[test]
    fn change_target_size() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 314, //change attribute
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new()
                .target_size(NonZeroU32::new(314).unwrap())
                .build()
        );
    }

    #[test]
    fn change_invert() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: true, //change attribute
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().invert(true).build()
        );
    }

    #[test]
    fn change_border() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: true, //change attribute
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().border(true).build()
        );
    }

    #[test]
    fn change_dimension() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Height, //change attribute
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new()
                .dimension(util::ResizingDimension::Height)
                .build()
        );
    }

    #[test]
    fn change_transform_x() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: true, //change attribute
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().transform_x(true).build()
        );
    }

    #[test]
    fn change_transform_y() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: true, //change attribute
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().transform_y(true).build()
        );
    }

    #[test]
    fn change_outline() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: true, //change attribute
                hysteresis: false,
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().outline(true).build()
        );
    }

    #[test]
    fn change_hysteresis() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: true, //change attribute
                target: ConversionTargetType::default(),
            },
            ConversionOptionBuilder::new().hysteresis(true).build()
        );
    }

    #[test]
    fn change_file_type() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                invert: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
                outline: false,
                hysteresis: false,
                target: ConversionTargetType::AnsiFile(false), //change attribute
            },
            ConversionOptionBuilder::new()
                .target(ConversionTargetType::AnsiFile(false))
                .build()
        );
    }
}
