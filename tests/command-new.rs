use assert_cmd::prelude::*;

use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, get_root_path, init_test, DELIMITER};

// It should create a new library with the default Cargo.toml file.
#[test]
fn new_has_correct_cargo_toml() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let cargo_toml =
    read_to_string("platformer_modules/Cargo.toml").expect("Unable to read Cargo.toml");
  let cargo_toml_split = cargo_toml.split("\n").collect::<Vec<&str>>();

  assert_eq!(cargo_toml_split[6], "[lib]");
  assert_eq!(cargo_toml_split[7], "crate-type = [\"cdylib\"]");
  assert_eq!(cargo_toml_split[9], "[dependencies]");
  assert_eq!(cargo_toml_split[10], "gdnative = \"0.9.1\"");
  assert_eq!(
    cargo_toml_split[11],
    "godot_rust_helper_ext = { git = \"https://github.com/robertcorponoi/godot_rust_helper_ext\" }"
  );

  cleanup_test_files();

  Ok(())
}

// It should create a new library with the default targets set in the config.
#[test]
fn new_has_correct_default_config() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let config =
    read_to_string("platformer_modules/godot-rust-helper.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  let gdnlib_path = Path::new("platformer/platformer_modules.gdnlib");

  assert_eq!(config_split[1], "name = \"platformer_modules\"");
  assert_eq!(config_split[2], "targets = [\"windows\"]");
  assert_eq!(config_split[3], "modules = []");
  assert_eq!(config_split[4], "plugin = false");
  assert_eq!(config_split[5], "");
  assert_eq!(config_split[6], "[paths]");
  assert_eq!(
    config_split[7],
    format!(
      "lib = \"{}{}tests{}platformer_modules\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[8],
    format!(
      "godot = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[9],
    format!(
      "output = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[10],
    format!(
      "nativescript = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );

  assert_eq!(gdnlib_path.exists(), true);

  cleanup_test_files();

  Ok(())
}

// It should create a new library with a gdnlib containing the default targets.
#[test]
fn new_has_correct_gdnlib_default_targets() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd.arg("new").arg("platformer_modules").arg("platformer");

  cmd.assert().success();

  let gdnlib =
    read_to_string("platformer/platformer_modules.gdnlib").expect("Unable to read gdnlib");
  let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

  assert_eq!(gdnlib_split[0], "[entry]");
  assert_eq!(gdnlib_split[1], "");
  assert_eq!(
    gdnlib_split[2],
    "Windows.64=\"res://platformer_modules.dll\""
  );
  assert_eq!(gdnlib_split[3], "");
  assert_eq!(gdnlib_split[4], "[dependencies]");
  assert_eq!(gdnlib_split[5], "");
  assert_eq!(gdnlib_split[6], "Windows.64=[  ]");
  assert_eq!(gdnlib_split[7], "");
  assert_eq!(gdnlib_split[8], "[general]");
  assert_eq!(gdnlib_split[9], "");
  assert_eq!(gdnlib_split[10], "singleton=false");
  assert_eq!(gdnlib_split[11], "load_once=true");
  assert_eq!(gdnlib_split[12], "symbol_prefix=\"godot_\"");
  assert_eq!(gdnlib_split[13], "reloadable=true");

  cleanup_test_files();

  Ok(())
}

// It should create a library and place the gdnlib file in the specified output folder.
#[test]
fn new_specify_output_correct_gdnlib_location() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--output-path")
    .arg("platformer/godot-rust-helper-output");

  cmd.assert().success();

  let gdnlib = read_to_string("platformer/godot-rust-helper-output/platformer_modules.gdnlib")
    .expect("Unable to read gdnlib");
  let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

  assert_eq!(gdnlib_split[0], "[entry]");
  assert_eq!(gdnlib_split[1], "");
  assert_eq!(
    gdnlib_split[2],
    "Windows.64=\"res://godot-rust-helper-output/platformer_modules.dll\""
  );
  assert_eq!(gdnlib_split[3], "");
  assert_eq!(gdnlib_split[4], "[dependencies]");
  assert_eq!(gdnlib_split[5], "");
  assert_eq!(gdnlib_split[6], "Windows.64=[  ]");
  assert_eq!(gdnlib_split[7], "");
  assert_eq!(gdnlib_split[8], "[general]");
  assert_eq!(gdnlib_split[9], "");
  assert_eq!(gdnlib_split[10], "singleton=false");
  assert_eq!(gdnlib_split[11], "load_once=true");
  assert_eq!(gdnlib_split[12], "symbol_prefix=\"godot_\"");
  assert_eq!(gdnlib_split[13], "reloadable=true");

  cleanup_test_files();

  Ok(())
}

// It should create a library and set a correct value for the nativescript output.
#[test]
fn new_specify_nativescript_correct_config() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--nativescript-path")
    .arg("platformer/godot-rust-helper-scripts");

  cmd.assert().success();

  let config =
    read_to_string("platformer_modules/godot-rust-helper.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  assert_eq!(config_split[1], "name = \"platformer_modules\"");
  assert_eq!(config_split[2], "targets = [\"windows\"]");
  assert_eq!(config_split[3], "modules = []");
  assert_eq!(config_split[4], "plugin = false");
  assert_eq!(config_split[5], "");
  assert_eq!(config_split[6], "[paths]");
  assert_eq!(
    config_split[7],
    format!(
      "lib = \"{}{}tests{}platformer_modules\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[8],
    format!(
      "godot = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[9],
    format!(
      "output = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[10],
    format!(
      "nativescript = \"{}{}tests{}platformer{}godot-rust-helper-scripts\"",
      get_root_path(),
      DELIMITER,
      DELIMITER,
      DELIMITER
    )
  );

  cleanup_test_files();

  Ok(())
}

// It should create a new library with --targets=windows,linux,osx and include them in the config.
#[test]
fn new_has_correct_targets_config() -> Result<(), Box<dyn Error>> {
  init_test();
  
  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let config =
    read_to_string("platformer_modules/godot-rust-helper.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  assert_eq!(config_split[0], "[general]");
  assert_eq!(config_split[1], "name = \"platformer_modules\"");
  assert_eq!(
    config_split[2],
    "targets = [\"windows\", \"linux\", \"osx\"]"
  );
  assert_eq!(config_split[3], "modules = []");
  assert_eq!(config_split[4], "plugin = false");
  assert_eq!(config_split[5], "");
  assert_eq!(config_split[6], "[paths]");
  assert_eq!(
    config_split[7],
    format!(
      "lib = \"{}{}tests{}platformer_modules\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[8],
    format!(
      "godot = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[9],
    format!(
      "output = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[10],
    format!(
      "nativescript = \"{}{}tests{}platformer\"",
      get_root_path(),
      DELIMITER,
      DELIMITER
    )
  );

  cleanup_test_files();

  Ok(())
}

// It should create a new library with --targets=windows,linux-osx and include them in the gdnlib.
#[test]
fn new_has_correct_gdnlib_all_targets() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let gdnlib =
    read_to_string("platformer/platformer_modules.gdnlib").expect("Unable to read gdnlib");
  let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

  assert_eq!(gdnlib_split[0], "[entry]");
  assert_eq!(gdnlib_split[1], "");
  assert_eq!(
    gdnlib_split[2],
    "OSX.64=\"res://libplatformer_modules.dylib\""
  );
  assert_eq!(gdnlib_split[3], "X11.64=\"res://libplatformer_modules.so\"");
  assert_eq!(
    gdnlib_split[4],
    "Windows.64=\"res://platformer_modules.dll\""
  );
  assert_eq!(gdnlib_split[5], "");
  assert_eq!(gdnlib_split[6], "[dependencies]");
  assert_eq!(gdnlib_split[7], "");
  assert_eq!(gdnlib_split[8], "OSX.64=[  ]");
  assert_eq!(gdnlib_split[9], "X11.64=[  ]");
  assert_eq!(gdnlib_split[10], "Windows.64=[  ]");
  assert_eq!(gdnlib_split[11], "");
  assert_eq!(gdnlib_split[12], "[general]");
  assert_eq!(gdnlib_split[13], "");
  assert_eq!(gdnlib_split[14], "singleton=false");
  assert_eq!(gdnlib_split[15], "load_once=true");
  assert_eq!(gdnlib_split[16], "symbol_prefix=\"godot_\"");
  assert_eq!(gdnlib_split[17], "reloadable=true");

  cleanup_test_files();

  Ok(())
}
