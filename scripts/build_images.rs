#!/usr/bin/env rust-script
//! Builds all Docker images for cross compilation.

use std::fs;
use std::process::Command;

fn main() {
    let image = "wolfgangwazzlestrauss/cross-sampitor";

    for entry in fs::read_dir("docker").unwrap() {
        let path = entry.unwrap().path();
        let target = path.extension().unwrap().to_str().unwrap();
        let tag = format!("{}:{}", image, target);
        
        let path_str = path.into_os_string().into_string().unwrap();
        Command::new("docker")
            .args(&["build", "-f", &path_str, "-t", &tag, "."])
            .spawn()
            .unwrap();
    }
}
