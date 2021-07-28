use core::result;
use log::*;
use rdkafka::ClientConfig;
use snap::read;
use snap::write;
use std::collections::HashMap;
use std::io::{Read, Write};

pub fn client_config(config_overrides: Option<HashMap<String, Option<String>>>) -> ClientConfig {
    let mut config = ClientConfig::new();
    if let Some(overrides) = config_overrides {
        for (key, value) in overrides {
            config.set(key, value.unwrap());
        }
    }
    config
}

pub fn frame_press(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut wtr = write::FrameEncoder::new(vec![]);
    if let Err(e) = wtr.write_all(bytes) {
        return Err(e.to_string());
    }
    match wtr.into_inner() {
        Err(e) => Err(e.to_string()),
        Ok(o) => Ok(o),
    }
}

pub fn compressed(bytes: &[u8]) -> SelResult<String> {
    match snap::raw::Encoder::new().compress_vec(bytes) {
        Err(e) => Err(e.to_string()),
        Ok(o) => {
            let out: String = o
                .iter()
                // .flat_map(|&b| b as char)
                .map(|&b| b as char)
                .collect();
            Ok(out)
        }
    }
}

pub type SelResult<T> = result::Result<T, String>;

pub fn decompress(bytes: &[u8]) -> SelResult<String> {
    match snap::raw::Decoder::new().decompress_vec(bytes) {
        Err(e) => Err(e.to_string()),
        Ok(o) => {
            let out: String = o
                .iter()
                // .flat_map(|&b| b as char)
                .map(|&b| b as char)
                .collect();
            Ok(out)
        }
    }
}

pub fn frame_press_old(bytes: &[u8]) -> Vec<u8> {
    let mut wtr = write::FrameEncoder::new(vec![]);
    wtr.write_all(bytes).unwrap();
    wtr.into_inner().unwrap()
}

pub fn frame_depress(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut buf = vec![];
    if let Err(e) = read::FrameDecoder::new(bytes).read_to_end(&mut buf) {
        return Err(e.to_string());
    }
    Ok(buf)
}

pub fn frame_depress_old(bytes: &[u8]) -> Vec<u8> {
    info!("frame_depress bytes:{:?}", bytes);
    let mut buf = vec![];
    read::FrameDecoder::new(bytes)
        .read_to_end(&mut buf)
        .unwrap();
    buf
}

pub fn to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub fn escape(bytes: &[u8]) -> String {
    use std::ascii::escape_default;
    bytes
        .iter()
        .flat_map(|&b| escape_default(b))
        .map(|b| b as char)
        .collect()
}
