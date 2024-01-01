use std::env;
use deko::CONFIG;

fn main()
{
  println!("➡️ starting deko_cli...");
  let image = env::current_dir()
    .unwrap()
    .join("m1-13-12-2022_20-53-26.jpg")
    .into_os_string()
    .into_string()
    .unwrap();
  let _ = deko::datagrams::jpeg::decode::decode_image(&image)
    .expect("failed to decode image");
  println!("☑️ deko_cli finished running!");
}