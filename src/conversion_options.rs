use crate::util::{self, ResizingDimension};

#[derive(Debug, PartialEq)]
pub struct ConversionOption<'a> {
    pub density: &'a str,
    pub thread_count: u32,
    pub scale: f64,
    pub target_size: u32,
    pub color: bool,
    pub invert: bool,
    pub on_background_color: bool,
    pub border: bool,
    pub dimension: ResizingDimension,
    pub transform: Option<ImageTransform>,
}

impl<'a> ConversionOption<'a> {
    // This method will help users to discover the builder
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
            },
            ConversionOption::builder()
        );
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct ConversionOptionBuilder<'a> {
    // Probably lots of optional fields.
    density: &'a str,
    thread_count: u32,
    scale: f64,
    target_size: u32,
    color: bool,
    invert: bool,
    on_background_color: bool,
    border: bool,
    dimension: ResizingDimension,
    transform: Option<ImageTransform>,
}

impl<'a> ConversionOptionBuilder<'a> {
    pub fn new(/* ... */) -> ConversionOptionBuilder<'a> {
        // These values will be all overwritten anyways
        ConversionOption::builder()
    }

    ///Set the used density
    pub fn density(mut self, density: &'a str) -> Self {
        self.density = density;
        self
    }

    ///Set the number of threads used to convert the image.
    ///Should be at least 1.
    pub fn thread_count(mut self, thread_count: u32) -> Self {
        self.thread_count = thread_count;
        self
    }

    ///Set the used scale.
    /// Used to change the ratio between width and height of an character.
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    ///Set the used target_size.
    /// Used to change the ratio between width and height of an character.
    pub fn target_size(mut self, target_size: u32) -> Self {
        self.target_size = target_size;
        self
    }

    ///Set if the img should be converted to colored chars.
    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    ///Invert to used density map.
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    ///Set the color to the ascii char background instead of the char directly.
    pub fn on_background(mut self, on_background: bool) -> Self {
        self.on_background_color = on_background;
        self
    }

    ///Enable the use of a border surrounding the image.
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    ///Set which dimension should be scaled first.
    pub fn dimension(mut self, dimension: util::ResizingDimension) -> Self {
        self.dimension = dimension;
        self
    }

    ///Change in which direction the image should be flipped.
    pub fn transform(mut self, transform: Option<ImageTransform>) -> Self {
        self.transform = transform;
        self
    }

    ///Build the ConversionOptions struct
    pub fn build(self) -> ConversionOption<'a> {
        ConversionOption {
            density: self.density,
            thread_count: self.thread_count,
            scale: self.scale,
            target_size: self.target_size,
            color: self.color,
            invert: self.invert,
            on_background_color: self.on_background_color,
            border: self.border,
            dimension: self.dimension,
            transform: self.transform,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
            },
            builder.build()
        );
    }

    #[test]
    fn change_thread_count() {
        let builder = ConversionOptionBuilder::new().thread_count(314);
        assert_eq!(
            ConversionOption {
                density: "",
                thread_count: 314, //change attribute
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 3.14f64, //change attribute
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
            },
            builder.build()
        );
    }

    #[test]
    fn change_target_size() {
        let builder = ConversionOptionBuilder::new().target_size(314);
        assert_eq!(
            ConversionOption {
                density: "",
                thread_count: 0,
                scale: 0.0f64,
                target_size: 314, //change attribute
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: true, //change attribute
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: true, //change attribute
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: true, //change attribute
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: true, //change attribute
                dimension: util::ResizingDimension::Width,
                transform: None,
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
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Height, //change attribute
                transform: None,
            },
            builder.build()
        );
    }

    #[test]
    fn change_transform() {
        let builder = ConversionOptionBuilder::new().transform(Some(ImageTransform::Y));
        assert_eq!(
            ConversionOption {
                density: "",
                thread_count: 0,
                scale: 0.0f64,
                target_size: 0,
                color: false,
                invert: false,
                on_background_color: false,
                border: false,
                dimension: util::ResizingDimension::Width,
                transform: Some(ImageTransform::Y), //change attribute
            },
            builder.build()
        );
    }
}

///Preferred image transform
///
///This flips the image along the x or y axis.
///By default X will be used.
#[derive(Debug, PartialEq)]
pub enum ImageTransform {
    X,
    Y,
    XY,
}
//Implement `Default` as Width
impl Default for ImageTransform {
    fn default() -> Self {
        ImageTransform::X
    }
}

#[cfg(test)]
mod test_image_transform_enum {
    use super::*;

    #[test]
    fn default_is_flip_x() {
        assert_eq!(ImageTransform::X, ImageTransform::default());
    }
}
