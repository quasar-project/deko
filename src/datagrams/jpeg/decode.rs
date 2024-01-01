use std::io::Read;
use std::mem::size_of;
use std::ops::Deref;
use anyhow::{
  anyhow,
  ensure,
  Error
};
use colored::Colorize;
use endian_codec::DecodeBE;
use crate::{CONFIG, log, warn};
use crate::datagrams::jpeg;
use crate::utils::checksum::Checksum;
use crate::utils::validate::Validate;

pub fn decode_image(path: &str) -> Result<jpeg::datagram::Metadata, Error>
{
  let cfg = CONFIG
    .lock()
    .unwrap();
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
  if !metadata.validate()? {
    warn!("metadata checksum mismatch, file may be corrupted!");
    warn!("expected crc: \t0x{:x}", metadata.checksum()?);
    warn!("actual crc: \t{}", format!("0x{:x}", metadata.checksum).bold().red());
    ensure!(cfg.allow_checksum_mismatch, "metadata checksum mismatch");
  }
  Ok(metadata)
}