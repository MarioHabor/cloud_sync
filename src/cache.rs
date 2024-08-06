use chrono::{DateTime, Local};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use crate::toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upload {
    pub file_path: String,
    pub last_saved: DateTime<Local>,
}

#[derive(Debug)]
pub struct Cache {
    data: HashMap<String, Upload>,
    cache_storage_path: String,
}

impl Cache {
    pub fn new(cache_storage_path: String) -> Self {
        let data = Self::load_from_file(&cache_storage_path).unwrap_or_default();
        Cache {
            data,
            cache_storage_path,
        }
    }

    pub fn add(&mut self, structure: Upload) {
        self.data.insert(structure.file_path.clone(), structure);
    }

    pub fn get(&self, file_path: &str) -> Option<&Upload> {
        self.data.get(file_path)
    }

    pub fn remove(&mut self, file_path: &str) -> Option<Upload> {
        self.data.remove(file_path)
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&self.data).unwrap();
        let mut file = File::create(&self.cache_storage_path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    fn load_from_file(cache_storage_path: &str) -> std::io::Result<HashMap<String, Upload>> {
        let cache_path = toml::parse_toml_cache_dir();

        if cache_storage_path.is_empty() {
            Self::create_cache(&cache_path);
        }

        let mut file = File::open(cache_storage_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let data: HashMap<String, Upload> = bincode::deserialize(&buffer).unwrap();
        Ok(data)
    }

    fn create_cache(cache_storage_path: &str) {
        let encoded: Vec<u8> = bincode::serialize("").unwrap();
        let mut file = File::create(cache_storage_path).expect("e");
        let _ = file.write_all(&encoded);
    }
}

pub fn main() -> std::io::Result<()> {
    let mut cache = Cache::new("cache.bin".to_string());

    // Check if a file exists in the cache
    if let Some(structure) = cache.get("/home/dev/Desktop/Master_Passworlds.kdbx") {
        println!(
            "Found existing file: {}, last checked: {}",
            structure.file_path, structure.last_saved
        );
    } else {
        println!("Existing file not found in cache");
    }

    cache.add(Upload {
        file_path: "/home/dev/Desktop/Master_Passworlds.kdbx".to_string(),
        last_saved: Local::now(),
    });

    // Add a new file to the cache
    let new_file = Upload {
        file_path: "new_file.txt".to_string(),
        last_saved: Local::now(),
    };
    cache.add(new_file);

    // Check a file
    let file_path = "/home/dev/Desktop/Master_Passworlds.kdbx";
    match cache.get(file_path) {
        Some(info) => {
            println!("File {} was last checked at {}", file_path, info.last_saved);
            // Decide if you need to check again based on info.last_saved
        }
        None => println!("File {} has never been checked before", file_path),
    }

    // Update the cache after checking
    cache.add(Upload {
        file_path: file_path.to_string(),
        last_saved: Local::now(),
    });

    // Save updated cache
    cache.save_to_file()?;

    Ok(())
}
