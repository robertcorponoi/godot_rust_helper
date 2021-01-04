use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test, BUILD_FILE_NAME, TARGET};

// It should create two modules and remove one.
#[test]
fn build_library() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--nativescript-path")
    .arg("platformer/godot-rust-helper-scripts")
    .arg(format!("--targets={}", TARGET));

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Hello")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("build")
    .output()
    .expect("Unable to execute cargo run");

  let build_file_name = format!("../platformer/{}", BUILD_FILE_NAME);
  let build_file_path = Path::new(&build_file_name);

  assert_eq!(build_file_path.exists(), true);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should place the build files in the correct specified output.
#[test]
fn build_specify_output_correct_build_file_location() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--output-path")
    .arg("platformer/godot-rust-helper-output")
    .arg("--nativescript-path")
    .arg("platformer/godot-rust-helper-scripts")
    .arg(format!("--targets={}", TARGET));

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Hello")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("build")
    .output()
    .expect("Unable to execute cargo run");

  let build_file_name = format!("../platformer/godot-rust-helper-output/{}", BUILD_FILE_NAME);
  let build_file_path = Path::new(&build_file_name);

  assert_eq!(build_file_path.exists(), true);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
