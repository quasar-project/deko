use std::env;

fn main()
{
  println!("➡️ starting cli...");
  deko::init_logger("trace")
    .expect("failed to initialize logger");

  let mut jpegs: Vec<String> = Vec::new();
  for entry in std::fs::read_dir(env::current_dir().unwrap()).unwrap() {
    let path = entry.unwrap().path();
    if path.is_file() {
      let ext = path.extension();
      if ext.is_some() && ext.unwrap() == "jpg" {
        jpegs.push(path.into_os_string().into_string().unwrap());
      }
    }
  }

  let mut decoder = deko::decoder::jpeg_decoder::JPEG_DECODER
    .lock()
    .unwrap();
  decoder.set_config(deko::Config::default());
  for jpeg in jpegs {
    decoder.decode_file(&jpeg).unwrap();
  }
  println!("☑️ cli finished running!");
}