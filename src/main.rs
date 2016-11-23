//! A user-configurable mathematical image generator

#[macro_use]
extern crate log;

extern crate env_logger;
extern crate hlua;
extern crate image;

mod imagebuilder;
mod generator;

use generator::{Generator, GeneratorError};
use image::DynamicImage;
use imagebuilder::ImageBuilder;
use std::fs::File;

fn construct_image<'a>(generator: &'a mut Generator) -> Result<DynamicImage, GeneratorError> {
  let width = generator.get_unsigned_32("width").expect("Width required.");
  let height = generator.get_unsigned_32("height").expect("Height required.");

  let mut image_builder = ImageBuilder::new(width, height)
    .blur(generator.get_float_32("blur"))
    .contrast_adj(generator.get_float_32("contrast_adj"))
    .brighten(generator.get_integer_32("brighten"));
  image_builder = match generator.get_boolean("grayscale") {
    Some(v) => image_builder.grayscale(v),
    None => image_builder,
  };
  image_builder = match generator.get_boolean("invert") {
    Some(v) => image_builder.invert(v),
    None => image_builder,
  };
  image_builder = match generator.get_boolean("flip_v") {
    Some(v) => image_builder.flip_v(v),
    None => image_builder,
  };
  image_builder = match generator.get_boolean("flip_h") {
    Some(v) => image_builder.flip_h(v),
    None => image_builder,
  };
  image_builder = match generator.get_unsigned_16("rotate") {
    Some(v) => image_builder.rotate(v),
    None => image_builder,
  };

  image_builder.build(generator)
}

fn main() {
  // Init logging
  env_logger::init().unwrap();

  // Init Lua
  let input_file = std::env::args().nth(1).expect("An input file is required");
  let input = File::open(input_file).unwrap();
  let mut generator = generator::Generator::ready(input).unwrap();

  // Generate Image
  let image = construct_image(&mut generator).unwrap();

  let output_file = generator.get_string("output")
    .unwrap_or(String::from("output.png"));
  let mut f = File::create(output_file).unwrap();
  image.save(&mut f, image::ImageFormat::PNG).unwrap();
}
