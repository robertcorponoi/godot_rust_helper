use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, init_test};

// It should remove all traces of a created module.
#[test]
fn destroy_remove_created_module() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--output-path")
    .arg("platformer/godot-rust-helper-output")
    .arg("--nativescript-path")
    .arg("platformer/godot-rust-helper-scripts");

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
    .arg("destroy")
    .arg("Hello")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  let config_file = read_to_string("godot-rust-helper.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  let mod_file_path = Path::new("src/hello.rs");

  let hello_gdns_file = Path::new("../platformer/godot-rust-helper-scripts/hello.gdns");

  assert_eq!(lib_file_split[0], "#[macro_use]");
  assert_eq!(lib_file_split[1], "extern crate gdnative;");
  assert_eq!(lib_file_split[2], "");
  assert_eq!(
    lib_file_split[3],
    "fn init(handle: gdnative::nativescript::InitHandle) {"
  );
  assert_eq!(lib_file_split[4], "}");
  assert_eq!(lib_file_split[5], "");
  assert_eq!(lib_file_split[6], "godot_init!(init);");
  assert_eq!(config_split[3], "modules = []");
  assert_eq!(mod_file_path.exists(), false);
  assert_eq!(hello_gdns_file.exists(), false);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create two modules and remove one.
#[test]
fn destroy_create_two_remove_one() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--output-path")
    .arg("platformer/godot-rust-helper-output")
    .arg("--nativescript-path")
    .arg("platformer/godot-rust-helper-scripts");

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
    .arg("create")
    .arg("World")
    .output()
    .expect("Unable to execute cargo run");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("destroy")
    .arg("World")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  let config_file = read_to_string("godot-rust-helper.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  let hello_mod_file_path = Path::new("src/hello.rs");
  let world_mod_file_path = Path::new("src/world.rs");

  let hello_gdns_file = Path::new("../platformer/godot-rust-helper-scripts/hello.gdns");
  let world_gdns_file = Path::new("../platformer/godot-rust-helper-scripts/world.gdns");

  assert_eq!(lib_file_split[0], "#[macro_use]");
  assert_eq!(lib_file_split[1], "extern crate gdnative;");
  assert_eq!(lib_file_split[2], "");
  assert_eq!(lib_file_split[3], "mod hello;");
  assert_eq!(lib_file_split[4], "");
  assert_eq!(
    lib_file_split[5],
    "fn init(handle: gdnative::nativescript::InitHandle) {"
  );
  assert_eq!(lib_file_split[6], "\thandle.add_class::<hello::Hello>();");
  assert_eq!(lib_file_split[7], "}");
  assert_eq!(lib_file_split[8], "");
  assert_eq!(lib_file_split[9], "godot_init!(init);");

  assert_eq!(config_split[3], "modules = [\"Hello\"]");

  assert_eq!(hello_mod_file_path.exists(), true);
  assert_eq!(world_mod_file_path.exists(), false);

  assert_eq!(hello_gdns_file.exists(), true);
  assert_eq!(world_gdns_file.exists(), false);

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
