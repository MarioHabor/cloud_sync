use hashbrown::HashMap;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Debug, Deserialize)]
struct TomlData {
    upload: HashMap<String, Upload>,
    cache_dir: CacheDir,
    cloud_providers: HashMap<String, CloudProviders>,
}

#[derive(Debug, Deserialize)]
struct CacheDir {
    dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    pub file_name: String,
    pub file_dir: String,
    pub veracrypt_mount_dir: String,
    pub veracrypt_file_name: String,
    pub veracrypt_volume_pw: String,
}

#[derive(Debug, Deserialize)]
pub struct CloudProviders {
    pub cloud_name: String,
    pub dir: String,
    pub paste_to_dir: String,
}

fn parse_toml() -> TomlData {
    match fs::read_to_string("upload.toml") {
        Ok(c) => {
            let _data: TomlData = match toml::from_str(&c) {
                // If successful, return data as `Data` struct.
                // `d` is a local variable.
                Ok(d) => return d,
                // Handle the `error` case.
                Err(_) => {
                    // Write `msg` to `stderr`.
                    eprintln!("Unable to load data from upload.toml");
                    // Exit the program with exit code `1`.
                    exit(1);
                }
            };
        }
        Err(_) => {
            println!("Could not read file");
            exit(1);
        }
    };
}

pub fn parse_toml_upload_list() -> HashMap<String, Upload> {
    let data: TomlData = parse_toml();

    // dbg!(&data.upload);

    data.upload
}

pub fn parse_toml_cache_dir() -> String {
    let data: TomlData = parse_toml();

    data.cache_dir.dir
}

pub fn parse_toml_cloud_providers() -> HashMap<String, CloudProviders> {
    let data: TomlData = parse_toml();

    data.cloud_providers
}
