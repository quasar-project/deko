use anyhow::ensure;
use image::GenericImageView;
use imageproc::point::Point;

pub fn load_image(data: &[u8]) -> anyhow::Result<image::DynamicImage>
{
  Ok(image::load_from_memory(data)?)
}

pub fn cut_image(image: &image::DynamicImage, x0: f32, lx: f32, div: f32, div_correction: f32)
  -> anyhow::Result<image::DynamicImage>
{
  ensure!(div_correction < div, "div_correction should be less than div");

  let (width, height) = image.dimensions();
  let (width, height) = (width as f32, height as f32);
  let mut image = image.clone().into_luma_alpha8();
  let top: Vec<Point<i32>> = vec![
    Point::new(0, ((height / 2.0f32) - 2.0 * x0 * ((div - div_correction) / 2.0f32).to_radians().tan()) as i32),
    Point::new(0, 0),
    Point::new(lx as i32, 0),
    Point::new(lx as i32, ((height / 2.0f32) - (2.0 * x0 + lx) * ((div - div_correction) / 2.0f32).to_radians().tan()) as i32)
  ];
  let bot: Vec<Point<i32>> = vec![
    Point::new(0, ((height / 2.0f32) + 2.0 * x0 * ((div - div_correction) / 2.0f32).to_radians().tan()) as i32),
    Point::new(0, height as i32),
    Point::new(lx as i32, height as i32),
    Point::new(lx as i32, ((height / 2.0f32) + (2.0 * x0 + lx) * ((div - div_correction) / 2.0f32).to_radians().tan()) as i32)
  ];
  imageproc::drawing::draw_polygon_mut(&mut image, top.as_slice(), image::LumaA([0, 0]));
  imageproc::drawing::draw_polygon_mut(&mut image, bot.as_slice(), image::LumaA([0, 0]));
  Ok(image::DynamicImage::from(image))
}