use std::env;

fn main()
{
  println!("➡️ starting cli...");
  deko::init_logger("trace")
    .expect("failed to initialize logger");
  let path = env::current_dir()
    .unwrap()
    .join("m1-13-12-2022_20-53-26.jpg")
    .into_os_string()
    .into_string()
    .unwrap();
  let decoder = deko::decoder::jpeg_decoder::JpegDecoder::new()
    .expect("failed to create decoder");
  decoder
    .decode_file(&path)
    .expect("failed to decode image");
  println!("☑️ cli finished running!");
}