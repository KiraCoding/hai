#!/usr/bin/env cargo

use std::process::Command;

fn main() {
    Command::new("cargo").args(&[]).status().expect();
}
