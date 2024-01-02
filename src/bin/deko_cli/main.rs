use std::env;
use deko::CONFIG;
use deko::image::{FromFile, StaticImage};

fn main()
{
  println!("➡️ starting deko_cli...");
  let path = env::current_dir()
    .unwrap()
    .join("m1-13-12-2022_20-53-26.jpg")
    .into_os_string()
    .into_string()
    .unwrap();
  let image = StaticImage::from_file(&path)
    .expect("deko_cli error");
  println!("☑️ deko_cli finished running!");
}