use core::str;
use rand::Rng;
use std::process::{Command, ExitStatus, Stdio};
// use std::{thread, time};

pub fn rclone_mount(cloud_name: &str, cloud_dir: &str) -> ExitStatus {
    let command = format!(
        r#"rclone mount --vfs-cache-mode writes --daemon {}: {}"#,
        cloud_name, cloud_dir
    );

    // println!("{}", &command);

    // Execute the command using sh -c
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // let three_sec = time::Duration::from_secs(3);
    // thread::sleep(three_sec);

    // println!("{}", &output.status.success());
    output.status
}

pub fn rclone_dismount(cloud_dir: &str) -> ExitStatus {
    let command = format!(r#"fusermount -u {}"#, cloud_dir);

    // Execute the command using sh -c
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // println!("{}", &output.status.success());
    // let three_sec = time::Duration::from_secs(3);
    // thread::sleep(three_sec);
    // dbg!(&output.status);
    output.status

    // Print the output of the command
    // if output.status.success() {
    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     println!("Output: {}", stdout);
    // } else {
    //     let stderr = String::from_utf8_lossy(&output.stderr);
    //     eprintln!("Error: {}", stderr);
    // }
}

pub fn mount_veracrypt(veracrypt_mount_dir: &str, veracrypt_volume_pw: &str) -> ExitStatus {
    let check_for_mount = r#"veracrypt --text --list"#;

    // Execute the command using sh -c
    let ch_output = Command::new("sh")
        .arg("-c")
        .arg(check_for_mount)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    if ch_output.status.success() {
        let stdout = String::from_utf8_lossy(&ch_output.stdout);
        if String::from(stdout.clone()).contains(&veracrypt_mount_dir) {
            return ch_output.status;
        }
    }

    let command = format!(
        r#"echo "1144" | sudo -S veracrypt --text --mount {} \
    /mnt --password {:?} --pim 0 --keyfiles "" --protect-hidden no --slot {} --verbose"#,
        veracrypt_mount_dir,
        veracrypt_volume_pw,
        rand::thread_rng().gen_range(5..64)
    );

    // Execute the command using sh -c
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // println!("{}", &output.status.success());
    output.status
}

pub fn dismount_veracrypt() -> ExitStatus {
    let command = r#"echo "1144" | sudo -S veracrypt --text --dismount /mnt"#;

    // Execute the command using sh -c
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    output.status
}

pub fn vera_remove_file(file_name: &str) -> ExitStatus {
    let rm_command = format!(r#"rclone deletefile "/mnt/{}""#, file_name);
    // Execute the command using sh -c
    let rm_output = Command::new("sh")
        .arg("-c")
        .arg(rm_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    rm_output.status
}

// pub fn remove_file(file_name: &str) -> ExitStatus {
//     let rm_command = format!(r#"rm {}"#, file_name);
//     // Execute the command using sh -c
//     let rm_output = Command::new("sh")
//         .arg("-c")
//         .arg(rm_command)
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .output()
//         .expect("Failed to execute command");
//
//     rm_output.status
// }

pub fn vera_copy_file(file_dir: &str) -> ExitStatus {
    let cp_command = format!(r#"rclone copy {:?} "/mnt/""#, file_dir);

    // Execute the command using sh -c
    let cp_output = Command::new("sh")
        .arg("-c")
        .arg(cp_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    cp_output.status
}

// pub fn copy_file(from: &str, to: &str) -> ExitStatus {
//     let cp_command = format!(r#"cp {} {}"#, from, to);
//
//     // Execute the command using sh -c
//     let cp_output = Command::new("sh")
//         .arg("-c")
//         .arg(cp_command)
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .output()
//         .expect("Failed to execute command");
//
//     cp_output.status
// }

pub fn rclone_copy_file(from: &str, to_remote: &str) -> ExitStatus {
    let cp_command = format!(r#"rclone copy {:?} {:?}"#, from, to_remote);

    // Execute the command using sh -c
    let cp_output = Command::new("sh")
        .arg("-c")
        .arg(cp_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    cp_output.status
}

pub fn rclone_delete_file(from_remote: &str, file_name: &str) -> ExitStatus {
    let cp_command = format!(r#"rclone deletefile "{}{}""#, from_remote, file_name);

    // Execute the command using sh -c
    let cp_output = Command::new("sh")
        .arg("-c")
        .arg(cp_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    cp_output.status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veracrypt_mount() {
        mount_veracrypt("/home/dev/Downloads/master-pws", "Marios96<#>");
    }

    #[test]
    fn test_rclone_mount() {
        rclone_mount("dg", "/home/dev/Documents/cloud/dg/");
        rclone_mount("od_rcl", "/home/dev/Documents/cloud/od/");
    }

    #[test]
    fn test_rclone_dismount() {
        rclone_dismount("/home/dev/Documents/cloud/dg/");
        rclone_dismount("/home/dev/Documents/cloud/od/");
    }

    #[test]
    fn test_veracrypt_dismount() {
        dismount_veracrypt();
    }
}
