use image::DynamicImage;

/// Loads a low resolution image.
///
/// The image is from <https://altphotos.com/photo/deaths-head-hawkmoth-3464/>
/// and is stored in the assets/images directory.
/// It has a resolution of 800x601.
///
/// # Examples
/// ```
/// use benchmarks::util;
/// let image = load_low_res_image();
/// assert_eq!((800, 601), image.dimensions());
/// ```
pub fn load_low_res_image() -> DynamicImage {
    let path = "assets/images/moth.jpg";
    load_image(path)
}

/// Loads a normal resolution image.
///
/// The image is from <https://upload.wikimedia.org/wikipedia/commons/thumb/a/ab/Abraham_Lincoln_O-77_matte_collodion_print.jpg/800px-Abraham_Lincoln_O-77_matte_collodion_print.jpg>
/// and is stored in the assets/images directory.
/// It has a resolution of 2850x3742.
///
/// # Examples
/// ```
/// use benchmarks::util;
/// let image = load_normal_res_image();
/// assert_eq!((2850, 3742), image.dimensions());
/// ```
pub fn load_normal_res_image() -> DynamicImage {
    let path = "assets/images/abraham_lincoln.jpg";
    load_image(path)
}

/// Loads a high resolution image.
///
/// The image is from <https://unsplash.com/photos/hDXk9iOi9bM>
/// and is stored in the assets/images directory.
/// It has a resolution of 3591x5386.
///
/// # Examples
/// ```
/// use benchmarks::util;
/// let image = load_high_res_image();
/// assert_eq!((3591, 5386), image.dimensions());
/// ```
pub fn load_high_res_image() -> DynamicImage {
    let path = "assets/images/radio_tower.jpg";
    load_image(path)
}

/// Load and returns the image from the given path.
///
/// # Panic
/// Panics when failing to open the image.
///
/// # Examples
/// ```
/// let image = load_image("test.png");
/// ```
fn load_image(path: impl AsRef<std::path::Path>) -> DynamicImage {
    let image = image::open(&path);

    if image.is_ok() {
        image.unwrap()
    } else {
        panic!("Failed to load image: {}", path.as_ref().to_str().unwrap())
    }
}
