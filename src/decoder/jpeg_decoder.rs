use std::env;
use std::mem::size_of;
use std::path::Path;
use anyhow::ensure;
use colored::Colorize;
use endian_codec::DecodeBE;
use log::{debug, warn};
use crate::config::Config;
use crate::decoder::image::metadata;
use crate::utils::{Checksum, Validate};

const DEFAULT_TARGET_DIRECTORY: &str = "cache";

pub struct JpegDecoder
{
  config: Config,
  target_directory: String
}

impl JpegDecoder
{
  pub fn new() -> anyhow::Result<Self>
  {
    Ok(Self
    {
      config: Config::default(),
      target_directory: env::current_dir()?
        .join(DEFAULT_TARGET_DIRECTORY)
        .into_os_string()
        .into_string()
        .unwrap()
    })
  }

  pub fn decode_data(&self, data: &mut [u8], filename: &str) -> anyhow::Result<()>
  {
    let metadata = self.extract_metadata(
      &mut data[..self.config.decoder_config.max_metadata_length]
    )?;
    Ok(())
  }

  pub fn decode_file(&self, path: &str) -> anyhow::Result<()>
  {
    let mut data = std::fs::read(path)?;
    let filename = Path::new(path)
      .file_name()
      .unwrap()
      .to_os_string()
      .into_string()
      .unwrap();
    self.decode_data(&mut data, &filename)
  }

  fn extract_metadata(&self, chunk: &mut [u8]) -> anyhow::Result<metadata::jpeg_metadata::Metadata>
  {
    let buf = &mut chunk[self.config.decoder_config.header_offset..];
    let header = metadata::jpeg_metadata::MetadataHeader::decode_from_be_bytes(
      &buf[..size_of::<metadata::jpeg_metadata::MetadataHeader>()]
    );

    ensure!(header.length as usize <= buf.len(),
      format!("invalid metadata length: {}, expected <={}",
        header.length,
        buf.len()
    ));
    ensure!(header.marker == self.config.decoder_config.metadata_marker,
      format!("invalid metadata marker: 0x{:x}, expected 0x{:x}",
        header.marker,
        self.config.decoder_config.metadata_marker
    ));

    let (meta, _): (metadata::jpeg_metadata::Metadata, usize) = bincode::decode_from_slice(
      &mut buf[size_of::<metadata::jpeg_metadata::MetadataHeader>()..header.length as usize],
      bincode::config::standard()
    )?;
    debug!("metadata header: {}", header.to_string().white().bold());
    debug!("metadata: {}", meta.to_string().green().bold());
    if !meta.validate()? {
      ensure!(self.config.general_config.allow_checksum_mismatch, "checksum mismatch in jpeg metadata");
      warn!("metadata checksum mismatch, file may be corrupted!");
      warn!("expected crc: \t0x{:x}", meta.checksum()?);
      warn!("actual crc: \t\t{}", format!("0x{:x}", meta.checksum).bold().red());
    }

    match self.config.general_config.fix_nans {
      true => Ok(meta.with_fixed_nans()),
      false => Ok(meta),
    }
  }
}