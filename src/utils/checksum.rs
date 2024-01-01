use anyhow::Error;

pub fn crc16(data: &[u8]) -> u16
{
  let mut crc: u16 = 0xFFFF;
  for d in data {
    crc ^= *d as u16;
    for _i in 0..8 {
      if (crc & 0x0001) != 0 {
        crc >>= 1;
        crc ^= 0xA001;
      } else {
        crc >>= 1;
      }
    }
  }
  crc
}

pub trait Checksum<T>
{
  fn checksum(&self) -> Result<T, Error>;
}