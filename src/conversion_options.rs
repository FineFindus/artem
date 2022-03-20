use std::num::NonZeroU32;

use crate::util::{self, ResizingDimension};

///Configuration for the conversion of the image to the ascii image.
#[derive(Debug, PartialEq)]
pub struct ConversionOption<'a> {
    pub density: &'a str,
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

impl<'a> ConversionOption<'a> {
    /// Create [ConversionOptionBuilder] with default properties.
    ///
    /// # Examples
    /// ```
    /// let default = ConversionOption::builder();
    /// ```
    pub fn builder() -> ConversionOptionBuilder<'a> {
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
                density: "",
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
pub struct ConversionOptionBuilder<'a> {
    density: &'a str,
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

impl<'a> ConversionOptionBuilder<'a> {
    ///Create a new ConversionOptionBuilder.
    ///
    /// If an option is not specified, the rust default value will be used, unless specified otherwise.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// ```
    pub fn new(/* ... */) -> ConversionOptionBuilder<'a> {
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
    pub fn density(mut self, density: &'a str) -> Self {
        if density.is_empty() {
            return self;
        }
        self.density = density;
        self
    }

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
    pub fn threads(mut self, threads: NonZeroU32) -> Self {
        self.threads = threads.get();
        self
    }

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
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    /// Set the target_size.
    ///
    /// This is the number of characters that the resulting ascii image will be heigh/wide.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.target_size(80);
    /// ```
    pub fn target_size(mut self, target_size: NonZeroU32) -> Self {
        self.target_size = target_size.get();
        self
    }

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
    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

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
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    ///Set the color to the ascii char background instead of the char directly.
    ///
    /// The density will determine how 'visible'/light/dark a character will be perceived.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.density("Mkl. ");
    /// ```
    pub fn on_background(mut self, on_background: bool) -> Self {
        self.background_color = on_background;
        self
    }

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
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
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
    pub fn dimension(mut self, dimension: util::ResizingDimension) -> Self {
        self.dimension = dimension;
        self
    }

    ///Flip the image along the X-axis
    ///
    /// This will flip the image horizontally by reversing reading of the horizontal part of the image.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.transform_x(true);
    /// ```
    pub fn transform_x(mut self, transform: bool) -> Self {
        self.transform_x = transform;
        self
    }

    ///Flip the image along the Y-axis
    ///
    /// This will flip the image vertically by reversing reading of the vertical part of the image.
    ///
    /// # Examples
    /// ```
    /// let mut builder = ConversionOptionBuilder::new();
    /// builder = builder.transform_y(true);
    /// ```
    pub fn transform_y(mut self, transform: bool) -> Self {
        self.transform_y = transform;
        self
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
    pub fn build(self) -> ConversionOption<'a> {
        ConversionOption {
            density: self.density,
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
                density: "",
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
        let builder = ConversionOptionBuilder::new().density("density");
        assert_eq!(
            ConversionOption {
                density: "density", //change attribute
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
            builder.build()
        );
    }

    #[test]
    fn change_thread_count() {
        let builder = ConversionOptionBuilder::new().threads(NonZeroU32::new(314).unwrap());
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_scale() {
        let builder = ConversionOptionBuilder::new().scale(3.14f64);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_target_size() {
        let builder = ConversionOptionBuilder::new().target_size(NonZeroU32::new(314).unwrap());
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_color() {
        let builder = ConversionOptionBuilder::new().color(true);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_invert() {
        let builder = ConversionOptionBuilder::new().invert(true);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_on_background_color() {
        let builder = ConversionOptionBuilder::new().on_background(true);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_border() {
        let builder = ConversionOptionBuilder::new().border(true);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_dimension() {
        let builder = ConversionOptionBuilder::new().dimension(util::ResizingDimension::Height);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_transform_x() {
        let builder = ConversionOptionBuilder::new().transform_x(true);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }

    #[test]
    fn change_transform_y() {
        let builder = ConversionOptionBuilder::new().transform_y(true);
        assert_eq!(
            ConversionOption {
                density: "",
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
            builder.build()
        );
    }
}
