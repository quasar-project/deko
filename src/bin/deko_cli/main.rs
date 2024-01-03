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
  let mut decoder = deko::decoder::jpeg_decoder::JPEG_DECODER
    .lock()
    .unwrap();
  decoder.set_config(deko::Config::default());
  decoder
    .decode_file(&path)
    .unwrap();
  println!("☑️ cli finished running!");
}