#!/usr/bin/env cargo +nightly -Zscript

//! ```cargo
//! [package]
//! edition = "2024"
//! ```

use std::env::args;
use std::fs::{copy, create_dir_all};
use std::io::{Error, ErrorKind, Result};
use std::process::Command;

const CODE: &str = "./tools/ovmf/x86_64/code.fd";
const VARS: &str = "./tools/ovmf/x86_64/vars.fd";

fn main() -> Result<()> {
    let target = args()
        .nth(1)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Missing target argument"))?;

    let build = Command::new("cargo")
        .args(&["build", "--profile", "qemu", "--target", &target])
        .status()?;

    if !build.success() {
        return Err(Error::new(ErrorKind::Other, "Build failed"));
    }

    create_dir_all("./target/x86_64-unknown-uefi/qemu/esp/efi/boot")?;
    copy(
        "./target/x86_64-unknown-uefi/qemu/kernel.efi",
        "./target/x86_64-unknown-uefi/qemu/esp/efi/boot/bootx64.efi",
    )?;

    Command::new("qemu-system-x86_64")
        .args(&[
            "-drive",
            &format!("if=pflash,format=raw,readonly=on,file={}", CODE),
            "-drive",
            &format!("if=pflash,format=raw,readonly=on,file={}", VARS),
            "-drive",
            "format=raw,file=fat:rw:./target/x86_64-unknown-uefi/qemu/esp",
        ])
        .status()?;

    Ok(())
}
