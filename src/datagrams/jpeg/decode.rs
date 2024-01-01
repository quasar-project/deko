use std::io::Read;
use std::mem::size_of;
use anyhow::{
  ensure,
  Error
};
use colored::Colorize;
use endian_codec::DecodeBE;
use crate::{CONFIG, log};
use crate::datagrams::jpeg;

pub fn decode_image(path: &str) -> Result<(), Error>
{
  let cfg = CONFIG
    .lock()
    .expect("failed to get config mutex");
  let mut file = std::fs::File::open(path)?;
  let mut buf = vec![0u8; cfg.jpeg.max_metadata_length];
  file.read(&mut buf)?;
  buf = buf[cfg.jpeg.header_offset..].to_vec();
  let header = jpeg::datagram::MetadataHeader::decode_from_be_bytes(
    &buf[..size_of::<jpeg::datagram::MetadataHeader>()]
  );

  ensure!(header.length as usize <= buf.len(), format!("invalid metadata length: {}, expected <={}",
    header.length,
    buf.len()
  ));
  ensure!(header.marker == cfg.jpeg.metadata_marker, format!("invalid metadata marker: 0x{:x}, expected 0x{:x}",
    header.marker,
    cfg.jpeg.metadata_marker
  ));

  let bc_cfg = bincode::config::standard();
  let (metadata, _): (jpeg::datagram::Metadata, usize) = bincode::decode_from_slice(
    &mut buf[size_of::<jpeg::datagram::MetadataHeader>()..header.length as usize],
    bc_cfg
  )?;
  log!("metadata header: {}", header.to_string().white().bold());
  log!("metadata: {}", metadata.to_string().green().bold());
  Ok(())
}