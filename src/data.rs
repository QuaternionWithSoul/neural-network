use std::path::Path;
use std::fs::{File, create_dir_all};
use serde::{Serialize, Deserialize};

use std::io::{Write, Read, Result, Error, ErrorKind};

use bincode;

use flate2::{write::GzEncoder, read::GzDecoder, Compression};

#[allow(dead_code)]
pub fn serialize<T: Serialize>(data: &T, path: &str, name: &str) -> Result<()> {
    let dir_path = Path::new(path);
    if !dir_path.exists() { create_dir_all(dir_path)?; }
    let file = File::create(dir_path.join(name))?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(&bincode::serialize(data).unwrap())?;
    encoder.finish()?;

    Ok(())
}

pub fn deserialize<T: for<'de> Deserialize<'de>>(path: &str, name: &str) -> Result<T> {
    let file = File::open(Path::new(path).join(name))?;
    let mut decoded_data = Vec::new();
    GzDecoder::new(file).read_to_end(&mut decoded_data)?;
    let data: T = bincode::deserialize(&decoded_data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(data)
}
