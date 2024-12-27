#!/usr/bin/env cargo

use std::fs::create_dir_all;
use std::process::Command;

const CODE: &str = "./tools/ovmf/x86_64/code.fd";
const VARS: &str = "./tools/ovmf/x86_64/vars.fd";

fn main() {
    let build = Command::new("cargo")
        .args(&[
            "build",
            "--profile",
            "qemu",
            "--target",
            "x86_64-unknown-uefi",
        ])
        .status()
        .unwrap();

    if !build.success() {
        return;
    }

    create_dir_all("./esp/efi/boot").unwrap();

    Command::new("qemu-system-x86_64")
        .args(&[
            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", CODE),
            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", VARS),
            "-drive", "format=raw,file=fat:rw:esp",
        ])
        .status()
        .unwrap();
}
