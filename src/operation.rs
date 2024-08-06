use crate::commands::*;
use chrono::{DateTime, Local};
use std::fs::{self};

use std::process::exit;

use crate::cache::*;

// type StoreDate = crate::cache::Upload;

pub fn check_last_update() -> std::io::Result<()> {
    let cache = load_cache();

    let toml = crate::toml::parse_toml_upload_list();

    for (_key, item) in toml {
        if exists_in_cache(cache.get(&item.file_dir)) {
            let last_saved = get_last_update_from_cache(cache.get(&item.file_dir.to_string()));

            println!(
                "{}: Last uploaded: {}",
                &item.file_dir,
                last_saved.format("%Y %B %d %H:%M:%S").to_string()
            );
        }
    }

    Ok(())
}

pub fn begin_upload() -> std::io::Result<()> {
    let cache = load_cache();

    let toml_upload_list = crate::toml::parse_toml_upload_list();

    for (_key, up_list_v) in toml_upload_list {
        if exists_in_cache(cache.get(&up_list_v.file_dir)) {
            let cache_last_update =
                get_last_update_from_cache(cache.get(&up_list_v.file_dir.to_string()));

            // check that the file has new modifications
            if compare_last_update(cache_last_update, up_list_v.file_dir.to_string()) {
                uoload(&up_list_v);
                println!("Uploaded: {}", &up_list_v.file_name);
            };
        } else {
            // else upload either way as it is a new file
            // that we do not have in cache
            uoload(&up_list_v);
            println!("Uploaded new file: {}", &up_list_v.file_name);
        }
    }

    Ok(())
}

fn uoload(up_list_v: &crate::toml::Upload) {
    let toml_cloud_providers = crate::toml::parse_toml_cloud_providers();

    mount_or_dismount_rcloud(true);

    mount_veracrypt(
        &up_list_v.veracrypt_mount_dir,
        &up_list_v.veracrypt_volume_pw,
    );
    vera_remove_file(&up_list_v.file_name);
    vera_copy_file(&up_list_v.file_dir);
    dismount_veracrypt();
    for (_k, c_v) in &toml_cloud_providers {
        format!("{}{}", &c_v.paste_to_dir, &up_list_v.veracrypt_file_name);

        let _ = rclone_delete_file(&c_v.paste_to_dir, &up_list_v.veracrypt_file_name);
        let _ = rclone_copy_file(&up_list_v.veracrypt_mount_dir, &c_v.paste_to_dir);
    }

    let _ = save_last_update_to_cache(&up_list_v.file_dir);
    mount_or_dismount_rcloud(false);
}

fn mount_or_dismount_rcloud(mount: bool) {
    let toml_cloud_providers = crate::toml::parse_toml_cloud_providers();
    // use std::{thread, time};

    for (_k, c_v) in &toml_cloud_providers {
        if mount {
            let code = rclone_mount(&c_v.cloud_name, &c_v.dir);
            if code.success() == true {
                continue;
            }
            break;
        } else {
            let code = rclone_dismount(&c_v.dir);
            if code.success() == true {
                continue;
            }
            break;
        }
    }
}

fn load_cache() -> Cache {
    let cache_dir: String = crate::toml::parse_toml_cache_dir();
    Cache::new(cache_dir)
}

fn exists_in_cache(file: Option<&Upload>) -> bool {
    if let Some(_structure) = file {
        return true;
    }
    println!("Existing file not found in cache");
    false
}

fn get_last_update_from_cache(file: Option<&Upload>) -> DateTime<Local> {
    if let Some(structure) = file {
        return structure.last_saved;
    }

    println!("Failed to read cache last saved");
    exit(1);
}

// Compares the time recorded for the file it was last updated
// To the modified time the file is located in the dir
fn compare_last_update(cache_time: DateTime<Local>, file_time: String) -> bool {
    match fs::metadata(file_time.as_str()) {
        Ok(data) => {
            if let Ok(modified) = data.modified() {
                let dt_file = to_epoch(modified.into());
                let dt_cache = to_epoch(cache_time.into());

                if dt_file > dt_cache {
                    return true;
                }
            } else {
                println!("Failed to read file modified date");
                exit(1);
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1);
        }
    }
    return false;
}

fn to_epoch(modified: DateTime<Local>) -> i64 {
    let datetime: DateTime<Local> = modified.into();
    datetime.timestamp()
}

fn save_last_update_to_cache(file_name: &str) -> std::io::Result<()> {
    let mut cache = load_cache();

    // Add a new file to the cache
    let new_file = Upload {
        file_path: file_name.to_string(),
        last_saved: Local::now(),
    };
    cache.add(new_file);

    // Save updated cache
    cache.save_to_file()?;

    Ok(())
}
