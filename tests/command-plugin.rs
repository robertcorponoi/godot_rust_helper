use assert_cmd::prelude::*;

use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{cleanup_test_files, get_root_path, init_test, DELIMITER};

// It should create a new plugin with the default Cargo.toml file.
#[test]
fn plugin_has_correct_cargo_toml() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob");

  cmd.assert().success();

  let cargo_toml =
    read_to_string("directory_browser/Cargo.toml").expect("Unable to read Cargo.toml");
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

// It should create a new plugin with the default targets set in the config.
#[test]
fn plugin_has_correct_default_config() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob");

  cmd.assert().success();

  let config =
    read_to_string("directory_browser/godot-rust-helper.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  let gdnlib_path = Path::new("platformer/addons/directory_browser/directory_browser.gdnlib");

  assert_eq!(config_split[1], "name = \"directory_browser\"");
  assert_eq!(config_split[2], "targets = [\"windows\"]");
  assert_eq!(config_split[3], "modules = [\"DirectoryBrowser\"]");
  assert_eq!(config_split[4], "plugin = true");
  assert_eq!(config_split[5], "");
  assert_eq!(config_split[6], "[paths]");
  assert_eq!(
    config_split[7],
    format!(
      "lib = \"{}{}tests{}directory_browser\"",
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
      "output = \"{}{}tests{}platformer{}addons{}directory_browser\"",
      get_root_path(),
      DELIMITER,
      DELIMITER,
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[10],
    format!(
      "nativescript = \"{}{}tests{}platformer{}addons{}directory_browser\"",
      get_root_path(),
      DELIMITER,
      DELIMITER,
      DELIMITER,
      DELIMITER
    )
  );

  assert_eq!(gdnlib_path.exists(), true);

  cleanup_test_files();

  Ok(())
}

// It should create a new plugin with a gdnlib containing the default targets.
#[test]
fn plugin_has_correct_gdnlib_default_targets() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob");

  cmd.assert().success();

  let gdnlib = read_to_string("platformer/addons/directory_browser/directory_browser.gdnlib")
    .expect("Unable to read gdnlib");
  let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

  assert_eq!(gdnlib_split[0], "[entry]");
  assert_eq!(gdnlib_split[1], "");
  assert_eq!(
    gdnlib_split[2],
    "Windows.64=\"res://addons/directory_browser/directory_browser.dll\""
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

// It should create a new plugin with --targets=windows,linux,osx and include them in the config.
#[test]
fn plugin_has_correct_targets_config() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let config =
    read_to_string("directory_browser/godot-rust-helper.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  assert_eq!(config_split[0], "[general]");
  assert_eq!(config_split[1], "name = \"directory_browser\"");
  assert_eq!(
    config_split[2],
    "targets = [\"windows\", \"linux\", \"osx\"]"
  );
  assert_eq!(config_split[3], "modules = [\"DirectoryBrowser\"]");
  assert_eq!(config_split[4], "plugin = true");
  assert_eq!(config_split[5], "");
  assert_eq!(config_split[6], "[paths]");
  assert_eq!(
    config_split[7],
    format!(
      "lib = \"{}{}tests{}directory_browser\"",
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
      "output = \"{}{}tests{}platformer{}addons{}directory_browser\"",
      get_root_path(),
      DELIMITER,
      DELIMITER,
      DELIMITER,
      DELIMITER
    )
  );
  assert_eq!(
    config_split[10],
    format!(
      "nativescript = \"{}{}tests{}platformer{}addons{}directory_browser\"",
      get_root_path(),
      DELIMITER,
      DELIMITER,
      DELIMITER,
      DELIMITER
    )
  );

  cleanup_test_files();

  Ok(())
}

// It should create a new plugin with --targets=windows,linux-osx and include them in the gdnlib.
#[test]
fn plugin_has_correct_gdnlib_all_targets() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let gdnlib = read_to_string("platformer/addons/directory_browser/directory_browser.gdnlib")
    .expect("Unable to read gdnlib");
  let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

  assert_eq!(gdnlib_split[0], "[entry]");
  assert_eq!(gdnlib_split[1], "");
  assert_eq!(
    gdnlib_split[2],
    "OSX.64=\"res://addons/directory_browser/libdirectory_browser.dylib\""
  );
  assert_eq!(
    gdnlib_split[3],
    "X11.64=\"res://addons/directory_browser/libdirectory_browser.so\""
  );
  assert_eq!(
    gdnlib_split[4],
    "Windows.64=\"res://addons/directory_browser/directory_browser.dll\""
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

// It should create a new plugin with the correct plugin.cfg file.
#[test]
fn plugin_has_correct_default_plugin_cfg_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let plugin_config = read_to_string("platformer/addons/directory_browser/plugin.cfg")
    .expect("Unable to read gdnlib");
  let plugin_config_split = plugin_config.split("\n").collect::<Vec<&str>>();

  assert_eq!(plugin_config_split[0], "[plugin]");
  assert_eq!(plugin_config_split[1], "name = \"Directory Browser\"");
  assert_eq!(plugin_config_split[2], "description = \"\"");
  assert_eq!(plugin_config_split[3], "author = \"\"");
  assert_eq!(plugin_config_split[4], "version = \"1.0\"");
  assert_eq!(
    plugin_config_split[5],
    "script = \"directory_browser.gdns\""
  );
  cleanup_test_files();

  Ok(())
}

// It should create a new plugin with the correct plugin.cfg file.
#[test]
fn plugin_has_correct_plugin_cfg_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob")
    .arg("--version")
    .arg("0.1.0")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let plugin_config = read_to_string("platformer/addons/directory_browser/plugin.cfg")
    .expect("Unable to read gdnlib");
  let plugin_config_split = plugin_config.split("\n").collect::<Vec<&str>>();

  assert_eq!(plugin_config_split[0], "[plugin]");
  assert_eq!(plugin_config_split[1], "name = \"Directory Browser\"");
  assert_eq!(plugin_config_split[2], "description = \"A test plugin\"");
  assert_eq!(plugin_config_split[3], "author = \"Bob\"");
  assert_eq!(plugin_config_split[4], "version = \"0.1.0\"");
  assert_eq!(
    plugin_config_split[5],
    "script = \"directory_browser.gdns\""
  );
  cleanup_test_files();

  Ok(())
}

// It should create a new plugin with the correct plugin base mod file.
#[test]
fn plugin_has_correct_plugin_base_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let plugin_base_gdns =
    read_to_string("directory_browser/src/directory_browser.rs").expect("Unable to read mod file");
  let plugin_base_gdns_split = plugin_base_gdns.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    plugin_base_gdns_split[0],
    "use gdnative::api::EditorPlugin;"
  );
  assert_eq!(
    plugin_base_gdns_split[1],
    "use gdnative::nativescript::user_data;"
  );
  assert_eq!(plugin_base_gdns_split[2], "");
  assert_eq!(plugin_base_gdns_split[3], "#[derive(NativeClass)]");
  assert_eq!(plugin_base_gdns_split[4], "#[inherit(EditorPlugin)]");
  assert_eq!(
    plugin_base_gdns_split[5],
    "#[user_data(user_data::LocalCellData<DirectoryBrowser>)]"
  );
  assert_eq!(plugin_base_gdns_split[6], "pub struct DirectoryBrowser;");
  assert_eq!(plugin_base_gdns_split[7], "");
  assert_eq!(plugin_base_gdns_split[8], "#[gdnative::methods]");
  assert_eq!(plugin_base_gdns_split[9], "impl DirectoryBrowser {");
  assert_eq!(
    plugin_base_gdns_split[10],
    "\tfn new(_owner: &EditorPlugin) -> Self {"
  );
  assert_eq!(plugin_base_gdns_split[11], "\t\tDirectoryBrowser");
  assert_eq!(plugin_base_gdns_split[12], "\t}");
  assert_eq!(plugin_base_gdns_split[13], "");
  assert_eq!(plugin_base_gdns_split[14], "\t#[export]");
  assert_eq!(
    plugin_base_gdns_split[15],
    "\tfn _ready(&self, _owner: &EditorPlugin) {"
  );
  assert_eq!(
    plugin_base_gdns_split[16],
    "\t\tgodot_print!(\"hello, world.\");"
  );
  assert_eq!(plugin_base_gdns_split[17], "\t}");
  assert_eq!(plugin_base_gdns_split[18], "}");
  assert_eq!(plugin_base_gdns_split[19], "");

  cleanup_test_files();

  Ok(())
}

// It should create a new plugin with the correct plugin gdns file.
#[test]
fn plugin_has_correct_plugin_gdns() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let plugin_base_gdns =
    read_to_string("platformer/addons/directory_browser/directory_browser.gdns")
      .expect("Unable to read gdnlib");
  let plugin_base_gdns_split = plugin_base_gdns.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    plugin_base_gdns_split[0],
    "[gd_resource type=\"NativeScript\" load_steps=2 format=2]"
  );
  assert_eq!(plugin_base_gdns_split[1], "");
  assert_eq!(
        plugin_base_gdns_split[2],
        "[ext_resource path=\"res://addons/directory_browser/directory_browser.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
  assert_eq!(plugin_base_gdns_split[3], "");
  assert_eq!(plugin_base_gdns_split[4], "[resource]");
  assert_eq!(plugin_base_gdns_split[5], "");
  assert_eq!(
    plugin_base_gdns_split[6],
    "resource_name = \"DirectoryBrowser\""
  );
  assert_eq!(
    plugin_base_gdns_split[7],
    "class_name = \"DirectoryBrowser\""
  );
  assert_eq!(plugin_base_gdns_split[8], "library = ExtResource( 1 )");
  assert_eq!(plugin_base_gdns_split[9], "");

  cleanup_test_files();

  Ok(())
}

// It should create a new plugin with with the correct src/lib.rs file.
#[test]
fn plugin_has_correct_lib_file() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("plugin")
    .arg("Directory Browser")
    .arg("directory_browser")
    .arg("platformer")
    .arg("--description")
    .arg("A test plugin")
    .arg("--author")
    .arg("Bob")
    .arg("--targets=windows,linux,osx");

  cmd.assert().success();

  let plugin_lib_file =
    read_to_string("directory_browser/src/lib.rs").expect("Unable to read lib file");
  let plugin_lib_file_split = plugin_lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(plugin_lib_file_split[3], "mod directory_browser;");
  assert_eq!(
    plugin_lib_file_split[6],
    "\thandle.add_tool_class::<directory_browser::DirectoryBrowser>();"
  );

  cleanup_test_files();

  Ok(())
}
