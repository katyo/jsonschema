use crate::{Error, Result};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn open_file(topic: &str, path: &Path) -> Result<File> {
    File::open(path).map_err(|error| {
        log::error!(
            "Unable to open {} file '{}' due to: {}",
            topic,
            path.display(),
            error
        );
        Error::Open
    })
}

pub fn read_input(topic: &str, path: &Path, input: &mut dyn Read) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    if let Err(error) = input.read_to_end(&mut data) {
        log::error!(
            "Unable to read {} from '{}' due to: {}",
            topic,
            path.display(),
            error
        );
        Err(Error::Read)
    } else {
        Ok(data)
    }
}

pub fn create_file(topic: &str, path: &Path) -> Result<File> {
    File::create(path).map_err(|error| {
        log::error!(
            "Unable to create {} file '{}' due to: {}",
            topic,
            path.display(),
            error
        );
        Error::Create
    })
}

pub fn write_output(
    topic: &str,
    path: &Path,
    output: &mut dyn Write,
    data: impl AsRef<[u8]>,
) -> Result<()> {
    let data = data.as_ref();
    if let Err(error) = output.write_all(&data) {
        log::error!(
            "Unable to write {} to '{}' due to: {}",
            topic,
            path.display(),
            error
        );
        Err(Error::Write)
    } else {
        Ok(())
    }
}

pub fn format_json(topic: &str, data: &json::Value, pretty: bool) -> Result<Vec<u8>> {
    if pretty {
        json::to_vec_pretty(data)
    } else {
        json::to_vec(data)
    }
    .map_err(|error| {
        log::error!("Unable to format {} due to: {}", topic, error);
        Error::Format
    })
}
