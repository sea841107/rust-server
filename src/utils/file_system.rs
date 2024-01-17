use std::fs;
use std::io::Error;

/** 讀取指定toml配置 */
pub fn read_toml<T>(path: &str) -> Result<T, Error> where T: serde::de::DeserializeOwned {
    let str = fs::read_to_string(path).unwrap();
    let map = toml::from_str(&str).unwrap();
    return Ok(map);
}