use image::*;
use super::generator::{Generator, GeneratorError};

/// A builder for `Image::DynamicImage`
pub struct ImageBuilder {
  width: u32,
  height: u32,
  grayscale: bool,
  invert: bool,
  blur: Option<f32>,
  unsharpen: Option<(f32, i32)>,
  contrast_adj: Option<f32>,
  brighten: Option<i32>,
  flip_v: bool,
  flip_h: bool,
  rotation: u16,
}

impl ImageBuilder {
  /// Creates a builder for a generated `Image::DynamicImage`
  pub fn new(width: u32, height: u32) -> ImageBuilder {
    ImageBuilder {
      width: width,
      height: height,
      grayscale: false,
      invert: false,
      blur: None,
      unsharpen: None,
      contrast_adj: None,
      brighten: None,
      flip_v: false,
      flip_h: false,
      rotation: 0,
    }
  }

  /// Make the generated image in grayscale
  pub fn grayscale(mut self, grayscale: bool) -> ImageBuilder {
    self.grayscale = grayscale;
    self
  }

  /// Make the colors on the generated image inverted
  pub fn invert(mut self, invert: bool) -> ImageBuilder {
    self.invert = invert;
    self
  }

  /// Gaussian blur the generated image by `sigma`
  pub fn blur(mut self, sigma: Option<f32>) -> ImageBuilder {
    self.blur = sigma;
    self
  }

  /// Brighten the generated image by `value`
  pub fn contrast_adj(mut self, contrast_adj: Option<f32>) -> ImageBuilder {
    self.contrast_adj = contrast_adj;
    self
  }

  /// Brighten the pixels of the generated image by `value`
  pub fn brighten(mut self, value: Option<i32>) -> ImageBuilder {
    self.brighten = value;
    self
  }

  /// Flip the generated image vertically
  pub fn flip_v(mut self, flip_v: bool) -> ImageBuilder {
    self.flip_v = flip_v;
    self
  }

  /// Flip the generated image horizontally
  pub fn flip_h(mut self, flip_h: bool) -> ImageBuilder {
    self.flip_h = flip_h;
    self
  }

  /// Rotate the image by `degrees` degrees.
  /// `degrees` must be a multiple of 90 degrees, and <= 360 degrees.
  pub fn rotate(mut self, degrees: u16) -> ImageBuilder {
    self.rotation = degrees;
    self
  }

  /// Generate and return the specified image
  pub fn build<'a>(&mut self,
                   generator: &'a mut Generator)
                   -> Result<DynamicImage, GeneratorError> {
    // Create the image
    info!("Creating image...");
    let mut image = DynamicImage::new_rgba8(self.width, self.height);

    // Generate color values for all pixels
    info!("Coloring pixels...");
    for x in 0..self.width {
      for y in 0..self.height {
        let pixel = Rgba {
          data: [generator.get_color_value("red", x, y)?,
                 generator.get_color_value("blue", x, y)?,
                 generator.get_color_value("green", x, y)?,
                 generator.get_color_value("alpha", x, y)?],
        };

        image.put_pixel(x, y, pixel);
      }
    }

    // Begin Postprocessing
    info!("Running postprocessing...");

    // Grayscale
    if self.grayscale {
      debug!("Applying grayscale...");
      image = image.grayscale();
    }

    // Invert
    if self.invert {
      debug!("Inverting colors...");
      image.invert();
    }

    // Blur
    if let Some(sigma) = self.blur {
      debug!("Bluring by {}...", sigma);
      image = image.blur(sigma);
    }

    // Unsharpen
    if let Some((sigma, threshold)) = self.unsharpen {
      debug!("Unsharpening (Σ: {}, threshold: {})...", sigma, threshold);
      image = image.unsharpen(sigma, threshold);
    }

    // Adjust Contrast
    if let Some(c) = self.contrast_adj {
      debug!("Adjusting contrast by {}...", c);
      image = image.adjust_contrast(c);
    }

    // Brighten
    if let Some(value) = self.brighten {
      debug!("Brightening by {}...", value);
      image = image.brighten(value);
    }

    // Flip Vertical
    if self.flip_v {
      debug!("Flipping vertically...");
      image = image.flipv();
    }

    // Flip Horizontal
    if self.flip_h {
      debug!("Flipping horizontally...");
      image = image.fliph();
    }

    // Rotate
    debug!("Attempting to rotate by {}°...", self.rotation);
    image = match self.rotation {
      0 | 360 => image,
      90 => image.rotate90(),
      180 => image.rotate180(),
      270 => image.rotate270(),
      _ => {
        panic!("Can not rotate {} - it is not a multiple of 90°!",
               self.rotation)
      }
    };

    info!("Image generated!");
    Ok(image)
  }
}
