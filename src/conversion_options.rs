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
    pub dimension: ResizingDimension,
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
                dimension: util::ResizingDimension::Width,
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
    dimension: ResizingDimension,
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
    ///Set which dimension should be scaled first.
    pub fn dimension(mut self, dimension: util::ResizingDimension) -> Self {
        self.dimension = dimension;
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
            dimension: self.dimension,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Width,
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
                dimension: util::ResizingDimension::Height, //change attribute
            },
            builder.build()
        );
    }
}
