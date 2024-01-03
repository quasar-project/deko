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
  let path1 = env::current_dir()
    .unwrap()
    .join("m1-13-12-2022_20-51-15.jpg")
    .into_os_string()
    .into_string()
    .unwrap();
  let path2 = env::current_dir()
    .unwrap()
    .join("m3-13-12-2022_21-26-14.jpg")
    .into_os_string()
    .into_string()
    .unwrap();
  let mut decoder = deko::decoder::jpeg_decoder::JPEG_DECODER
    .lock()
    .unwrap();
  decoder.set_config(deko::Config::default());
  decoder.decode_file(&path).unwrap();
  decoder.decode_file(&path1).unwrap();
  decoder.decode_file(&path2).unwrap();
  println!("☑️ cli finished running!");
}