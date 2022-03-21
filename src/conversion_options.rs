use std::num::NonZeroU32;

use crate::util::{self, ResizingDimension};

///Configuration for the conversion of the image to the ascii image.
#[derive(Debug, PartialEq)]
pub struct ConversionOption {
    pub density: String,
    pub threads: u32,
    pub scale: f64,
    pub target_size: u32,
    pub color: bool,
    pub invert: bool,
    pub background_color: bool,
    pub border: bool,
    pub dimension: ResizingDimension,
    pub transform_x: bool,
    pub transform_y: bool,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
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
    color: bool,
    invert: bool,
    background_color: bool,
    border: bool,
    dimension: ResizingDimension,
    transform_x: bool,
    transform_y: bool,
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
    /// builder = builder.density("Mkl. ");
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
    /// builder = builder.thread(4);
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
    /// builder = builder.scale(0.42f64);
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
    /// builder = builder.target_size(80);
    /// ```
    => target_size, NonZeroU32, get
    }

    property! {
    /// Set if the img should be converted to colored chars.
    ///
    /// By default truecolor chars will be used, if there are not supported,
    /// ANSI-Colors will be used as a fallback.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.color(true);
    /// ```
    => color, bool
    // pub fn color(mut self, color: bool) -> Self {
    //     self.color = color;
    //     self
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
    /// builder = builder.invert(true);
    /// ```
    => invert, bool
    // pub fn invert(mut self, invert: bool) -> Self {
    //     self.invert = invert;
    //     self
    }

    property! {
    ///Set the color to the ascii char background instead of the char directly.
    ///
    /// The density will determine how 'visible'/light/dark a character will be perceived.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.density("Mkl. ");
    /// ```
    => background_color, bool
    // pub fn on_background(mut self, on_background: bool) -> Self {
    //     self.background_color = on_background;
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
    /// builder = builder.border(true);
    /// ```
    => border, bool
    // pub fn border(mut self, border: bool) -> Self {
    //     self.border = border;
    //     self
    }

    /// Set which dimension should be scaled first.
    ///
    /// See [ResizingDimension] for more information.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.dimension(util::ResizingDimension:.Height);
    /// ```
    #[allow(clippy::needless_lifetimes)] //disable this, as the life is needed for the builder
    pub fn dimension<'a>(&'a mut self, dimension: util::ResizingDimension) -> &'a Self {
        self.dimension = dimension;
        self
    }

    property! {
    ///Flip the image along the X-axis
    ///
    /// This will flip the image horizontally by reversing reading of the horizontal part of the image.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.transform_x(true);
    /// ```
    => transform_x, bool
    // pub fn transform_x(mut self, transform: bool) -> Self {
    //     self.transform_x = transform;
    //     self
    }

    property! {
    ///Flip the image along the Y-axis
    ///
    /// This will flip the image vertically by reversing reading of the vertical part of the image.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.transform_y(true);
    /// ```
    => transform_y, bool
    // pub fn transform_y(mut self, transform: bool) -> Self {
    //     self.transform_y = transform;
    //     self
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
            color: self.color,
            invert: self.invert,
            background_color: self.background_color,
            border: self.border,
            dimension: self.dimension,
            transform_x: self.transform_x,
            transform_y: self.transform_y,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
            },
            ConversionOptionBuilder::new()
                .target_size(NonZeroU32::new(314).unwrap())
                .build()
        );
    }

    #[test]
    fn change_color() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                color: true, //change attribute
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
            },
            ConversionOptionBuilder::new().color(true).build()
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
                color: false,
                invert: true, //change attribute
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
            },
            ConversionOptionBuilder::new().invert(true).build()
        );
    }

    #[test]
    fn change_on_background_color() {
        assert_eq!(
            ConversionOption {
                density: String::new(),
                threads: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                background_color: true, //change attribute
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
            },
            ConversionOptionBuilder::new()
                .background_color(true)
                .build()
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
                color: false,
                invert: false,
                background_color: false,
                border: true, //change attribute
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Height, //change attribute
                transform_x: false,
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: true, //change attribute
                transform_y: false,
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
                color: false,
                invert: false,
                background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform_x: false,
                transform_y: true, //change attribute
            },
            ConversionOptionBuilder::new().transform_y(true).build()
        );
    }
}
