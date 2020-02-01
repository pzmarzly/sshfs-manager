#![feature(proc_macro_hygiene)]
use command_macros::command;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::Stdio;

fn main() {
    let success =
        |mut cmd: std::process::Command| cmd.stdout(Stdio::null()).status().unwrap().success();

    // sudo causes ~/ to become /root/
    let home_dir = std::env::var("HOME").unwrap();

    let remote = std::env::args()
        .skip(1)
        .next()
        .expect("No remote address provided!");
    let remote_hash = {
        let mut hasher = Sha256::new();
        hasher.input(remote.as_bytes());
        hasher.result()
    };
    let local_dir = format!("/mnt/sshfs_{:x}", remote_hash);

    if Path::new(&local_dir).is_dir() {
        println!("Unmounting {}", local_dir);
        if success(command!(sudo bash "-c" "umount "(local_dir)" && rmdir "(local_dir))) {
            println!("Success!");
        } else {
            println!("Failed!");
        }
    } else {
        println!("Mounting {} at {}", remote, local_dir);
        if success(command!(sudo bash "-c" "
            mkdir "(local_dir)" &&
            sshfs \
                -F "(home_dir)"/.ssh/config \
                -o allow_other \
                -o transform_symlinks \
                -o follow_symlinks \
                -o auto_cache \
                -o reconnect \
                -o ssh_command='ssh -i "(home_dir)"/.ssh/id_rsa' \
                "(remote)":/ "(local_dir)))
        {
            println!("Success!");
        } else {
            println!("Failed!");
        }
    }
}
