use std::collections::HashSet;
use std::{env, fs};
use std::fs::File;
use std::mem::size_of;
use std::path::Path;
use std::sync::Mutex;
use anyhow::ensure;
use colored::Colorize;
use endian_codec::DecodeBE;
use lazy_static::lazy_static;
use log::{debug, trace, warn};
use crate::Config;
use crate::decoder::image::metadata;
use crate::decoder::utility_extensions::Extension;
use crate::utils::{Checksum, Validate};

const DEFAULT_TARGET_DIRECTORY: &str = "cache";

lazy_static!
{
  pub static ref JPEG_DECODER: Mutex<JpegDecoder> = Mutex::new(
    JpegDecoder::new()
      .expect("failed to create jpeg decoder instance")
  );
}

pub struct JpegDecoder
{
  config: Config,
  target_directory: String,
  decoded_names: HashSet<String>
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
        .unwrap(),
      decoded_names: HashSet::new()
    })
  }

  pub fn set_config(&mut self, config: Config) -> &Self
  {
    self.config = config;
    self
  }

  pub fn decode_data(&mut self, data: &mut [u8], filename: &str) -> anyhow::Result<()>
  {
    ensure!(!self.decoded_names.contains(filename),
      format!("jpeg image {} has been already decoded", filename)
    );
    let metadata = self.extract_metadata(
      &mut data[..self.config.decoder_config.max_metadata_length]
    )?;
    self.save_to_json(&metadata, filename, Some("meta"))?;
    trace!("adding {} to decoded names list", filename);
    self.decoded_names.insert(String::from(filename));
    Ok(())
  }

  pub fn decode_file(&mut self, path: &str) -> anyhow::Result<()>
  {
    let mut data = std::fs::read(path)?;
    let filename = Path::new(path)
      .file_stem()
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

  fn save_to_json<T>(&self, data: &T, filename: &str, override_name: Option<&str>) -> anyhow::Result<()>
    where T: serde::Serialize + Sized
  {
    let json = serde_json::to_string_pretty(data)?;
    let file = self.filepath_from_filename(filename, Extension::Json, override_name)?;
    let file = File::create(file)?;
    trace!("saving metadata of {} to json file", filename);
    Ok(serde_json::to_writer_pretty(file, data)?)
  }

  fn directory_from_filename(&self, filename: &str) -> anyhow::Result<String>
  {
    let dir_path = Path::new(&self.target_directory)
      .join(filename);
    fs::create_dir_all(&dir_path)?;
    Ok(dir_path
      .into_os_string()
      .into_string()
      .unwrap()
    )
  }

  fn filepath_from_filename(&self, filename: &str, extension: Extension, override_name: Option<&str>)
    -> anyhow::Result<String>
  {
    let dir_path = self.directory_from_filename(filename)?;
    let file_path = Path::new(&dir_path)
      .join(override_name.unwrap_or_else(|| filename))
      .with_extension(extension.extension()?);
    Ok(file_path
      .into_os_string()
      .into_string()
      .unwrap()
    )
  }
}