#!/usr/bin/env rust-script
//! Tests external targets with cross.

use std::fs;
use std::process::Command;

fn main() {
    for entry in fs::read_dir("docker").unwrap() {
        let path = entry.unwrap().path();
        let target = path.extension().unwrap().to_str().unwrap();
        
        Command::new("cross")
            .args(&["test", "--target", &target])
            .spawn()
            .unwrap();
    }
}
