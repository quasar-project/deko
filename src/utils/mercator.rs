const MAP_SCALE_RATIO: f64 = 156543.03392;

pub fn mercator_zoom_level(latitude: f64, meters_per_pixel: f64) -> f64
{
  (MAP_SCALE_RATIO * latitude
    .to_radians()
    .cos() / if meters_per_pixel <= 0.0 { 1.0f64 } else { meters_per_pixel }
  ).log(2.0)
}