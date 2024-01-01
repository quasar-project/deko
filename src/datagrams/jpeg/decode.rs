use std::io::Read;
use anyhow::Error;
use crate::datagrams::jpeg;

// pub fn decode_image(path: &str) -> Result<jpeg::datagram::Metadata, Error>
// {
//   let mut file = std::fs::File::open(path)?;
//   let mut buf = vec![0u8; 1024];
//   file.read(&mut buf)?;
//
// }