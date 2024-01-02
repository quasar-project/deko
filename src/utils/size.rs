use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Size<T>
  where T: PartialEq + PartialOrd + Sized + Copy + Clone + Hash + Display
{
  pub width: T,
  pub height: T
}

impl<T> Display for Size<T>
  where T: PartialEq + PartialOrd + Sized + Copy + Clone + Hash + Display
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "({}, {})", self.width, self.height)
  }
}

impl<T> Size<T>
  where T: PartialEq + PartialOrd + Sized + Copy + Clone + Hash + Display
{
  pub fn new(width: T, height: T) -> Self
  {
    Self
    {
      width,
      height
    }
  }
}