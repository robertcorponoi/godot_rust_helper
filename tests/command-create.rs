use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::{read_to_string, remove_file};
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{
  cleanup_test_files, get_root_path, init_test, DELIMITER,
};

// It should create a module and add an entry for it in the config file.
#[test]
fn create_add_module_to_config() -> Result<(), Box<dyn Error>> {
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

  let config = read_to_string("godot-rust-helper.toml").expect("Unable to read config");
  let config_split = config.split("\n").collect::<Vec<&str>>();

  assert_eq!(config_split[0], "[general]");
  assert_eq!(config_split[1], "name = \"platformer_modules\"");
  assert_eq!(config_split[2], "targets = [\"windows\"]");
  assert_eq!(config_split[3], "modules = [\"Hello\"]");
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
      "output = \"{}{}tests{}platformer{}godot-rust-helper-output\"",
      get_root_path(),
      DELIMITER,
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

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create a module and add it to the lib file.
#[test]
fn create_add_module_to_lib() -> Result<(), Box<dyn Error>> {
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

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

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

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create a module and add create a module file for it.
#[test]
fn create_mod_file() -> Result<(), Box<dyn Error>> {
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

  let mod_file = read_to_string("src/hello.rs").expect("Unable to read module file");
  let mod_file_split = mod_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(mod_file_split[0], "use gdnative::api::Node;");
  assert_eq!(mod_file_split[1], "use gdnative::nativescript::user_data;");
  assert_eq!(mod_file_split[2], "");
  assert_eq!(mod_file_split[3], "#[derive(NativeClass)]");
  assert_eq!(mod_file_split[4], "#[inherit(Node)]");
  assert_eq!(
    mod_file_split[5],
    "#[user_data(user_data::LocalCellData<Hello>)]"
  );
  assert_eq!(mod_file_split[6], "pub struct Hello;");
  assert_eq!(mod_file_split[7], "");
  assert_eq!(mod_file_split[8], "#[gdnative::methods]");
  assert_eq!(mod_file_split[9], "impl Hello {");
  assert_eq!(mod_file_split[10], "\tfn new(_owner: &Node) -> Self {");
  assert_eq!(mod_file_split[11], "\t\tHello");
  assert_eq!(mod_file_split[12], "\t}");
  assert_eq!(mod_file_split[13], "");
  assert_eq!(mod_file_split[14], "\t#[export]");
  assert_eq!(mod_file_split[15], "\tfn _ready(&self, _owner: &Node) {");
  assert_eq!(mod_file_split[16], "\t\tgodot_print!(\"hello, world.\");");
  assert_eq!(mod_file_split[17], "\t}");
  assert_eq!(mod_file_split[18], "}");
  assert_eq!(mod_file_split[19], "");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create a module and add create a gdns file for it.
#[test]
fn create_gdns_file_at_default_location() -> Result<(), Box<dyn Error>> {
  init_test();

  let mut cmd = Command::cargo_bin("godot_rust_helper")?;
  cmd
    .arg("new")
    .arg("platformer_modules")
    .arg("platformer")
    .arg("--output-path")
    .arg("platformer/godot-rust-helper-output");

  cmd.assert().success();

  set_current_dir("platformer_modules").expect("Unable to change to library directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("Player")
    .output()
    .expect("Unable to execute cargo run");

  let gdns_file_contents =
    read_to_string("../platformer/player.gdns").expect("Unable to read gdns file");
  let gdns_file_contents_split = gdns_file_contents.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    gdns_file_contents_split[0],
    "[gd_resource type=\"NativeScript\" load_steps=2 format=2]"
  );
  assert_eq!(gdns_file_contents_split[1], "");
  assert_eq!(
        gdns_file_contents_split[2],
        "[ext_resource path=\"res://godot-rust-helper-output/platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
  assert_eq!(gdns_file_contents_split[3], "");
  assert_eq!(gdns_file_contents_split[4], "[resource]");
  assert_eq!(gdns_file_contents_split[5], "");
  assert_eq!(gdns_file_contents_split[6], "resource_name = \"Player\"");
  assert_eq!(gdns_file_contents_split[7], "class_name = \"Player\"");
  assert_eq!(gdns_file_contents_split[8], "library = ExtResource( 1 )");
  assert_eq!(gdns_file_contents_split[9], "");

  set_current_dir("../").expect("Unable to change to parent directory");

  remove_file("platformer/player.gdns").unwrap();

  cleanup_test_files();

  Ok(())
}

// It should create a module and add create a gdns file for it at the specified nativescript path.
#[test]
fn create_gdns_file_at_specified_nativescript_location() -> Result<(), Box<dyn Error>> {
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
    .arg("Player")
    .output()
    .expect("Unable to execute cargo run");

  let gdns_file_contents = read_to_string("../platformer/godot-rust-helper-scripts/player.gdns")
    .expect("Unable to read gdns file");
  let gdns_file_contents_split = gdns_file_contents.split("\n").collect::<Vec<&str>>();

  assert_eq!(
    gdns_file_contents_split[0],
    "[gd_resource type=\"NativeScript\" load_steps=2 format=2]"
  );
  assert_eq!(gdns_file_contents_split[1], "");
  assert_eq!(
        gdns_file_contents_split[2],
        "[ext_resource path=\"res://godot-rust-helper-output/platformer_modules.gdnlib\" type=\"GDNativeLibrary\" id=1]"
    );
  assert_eq!(gdns_file_contents_split[3], "");
  assert_eq!(gdns_file_contents_split[4], "[resource]");
  assert_eq!(gdns_file_contents_split[5], "");
  assert_eq!(gdns_file_contents_split[6], "resource_name = \"Player\"");
  assert_eq!(gdns_file_contents_split[7], "class_name = \"Player\"");
  assert_eq!(gdns_file_contents_split[8], "library = ExtResource( 1 )");
  assert_eq!(gdns_file_contents_split[9], "");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create a with multiple capital letters in the name.
#[test]
fn create_multiple_captial_letters() -> Result<(), Box<dyn Error>> {
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
    .arg("MainScene")
    .output()
    .expect("Unable to execute cargo run");

  let config_file = read_to_string("godot-rust-helper.toml").expect("Unable to read config file");
  let mod_file = read_to_string("src/main_scene.rs").expect("Unable to read module file");
  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");

  let mod_file_split = mod_file.split("\n").collect::<Vec<&str>>();
  let config_split = config_file.split("\n").collect::<Vec<&str>>();
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(mod_file_split[6], "pub struct MainScene;");
  assert_eq!(mod_file_split[9], "impl MainScene {");
  assert_eq!(mod_file_split[11], "\t\tMainScene");

  assert_eq!(config_split[3], "modules = [\"MainScene\"]");

  assert_eq!(lib_file_split[3], "mod main_scene;");
  assert_eq!(
    lib_file_split[6],
    "\thandle.add_class::<main_scene::MainScene>();"
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create multiple modules.
#[test]
fn create_multiple_modules() -> Result<(), Box<dyn Error>> {
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
    .arg("MainScene")
    .output()
    .expect("Unable to execute cargo run");

  let hello_file_path = Path::new("src/hello.rs");
  let main_scene_file_path = Path::new("src/main_scene.rs");
  let gdnlib_file_path =
    Path::new("../platformer/godot-rust-helper-output/platformer_modules.gdnlib");
  let hello_ns_file_path = Path::new("../platformer/godot-rust-helper-scripts/hello.gdns");
  let main_scene_ns_file_path =
    Path::new("../platformer/godot-rust-helper-scripts/main_scene.gdns");

  let config_file = read_to_string("godot-rust-helper.toml").expect("Unable to read config file");
  let config_split = config_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(hello_file_path.exists(), true);
  assert_eq!(main_scene_file_path.exists(), true);
  assert_eq!(gdnlib_file_path.exists(), true);
  assert_eq!(hello_ns_file_path.exists(), true);
  assert_eq!(main_scene_ns_file_path.exists(), true);
  assert_eq!(config_split[3], "modules = [\"Hello\", \"MainScene\"]");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create multiple modules and add them to the lib file.
#[test]
fn create_multiple_modules_and_add_to_lib() -> Result<(), Box<dyn Error>> {
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

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(lib_file_split[0], "#[macro_use]");
  assert_eq!(lib_file_split[1], "extern crate gdnative;");
  assert_eq!(lib_file_split[2], "");
  assert_eq!(lib_file_split[3], "mod hello;");
  assert_eq!(lib_file_split[4], "mod world;");
  assert_eq!(lib_file_split[5], "");
  assert_eq!(
    lib_file_split[6],
    "fn init(handle: gdnative::nativescript::InitHandle) {"
  );
  assert_eq!(lib_file_split[7], "\thandle.add_class::<hello::Hello>();");
  assert_eq!(lib_file_split[8], "\thandle.add_class::<world::World>();");
  assert_eq!(lib_file_split[9], "}");
  assert_eq!(lib_file_split[10], "");
  assert_eq!(lib_file_split[11], "godot_init!(init);");

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}

// It should create a module inside of a plugin.
#[test]
fn create_additional_plugin_script() -> Result<(), Box<dyn Error>> {
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

  set_current_dir("directory_browser").expect("Unable to change to plugin directory");
  Command::new("cargo")
    .arg("run")
    .arg("--manifest-path=../../Cargo.toml")
    .arg("create")
    .arg("FolderStructure")
    .output()
    .expect("Unable to execute cargo run");

  let lib_file = read_to_string("src/lib.rs").expect("Unable to read lib file");
  let lib_file_split = lib_file.split("\n").collect::<Vec<&str>>();

  assert_eq!(lib_file_split[0], "#[macro_use]");
  assert_eq!(lib_file_split[1], "extern crate gdnative;");
  assert_eq!(lib_file_split[2], "");
  assert_eq!(lib_file_split[3], "mod directory_browser;");
  assert_eq!(lib_file_split[4], "mod folder_structure;");
  assert_eq!(lib_file_split[5], "");
  assert_eq!(
    lib_file_split[6],
    "fn init(handle: gdnative::nativescript::InitHandle) {"
  );
  assert_eq!(
    lib_file_split[7],
    "\thandle.add_tool_class::<directory_browser::DirectoryBrowser>();"
  );
  assert_eq!(
    lib_file_split[8],
    "\thandle.add_tool_class::<folder_structure::FolderStructure>();"
  );
  assert_eq!(lib_file_split[9], "}");
  assert_eq!(lib_file_split[10], "");
  assert_eq!(lib_file_split[11], "godot_init!(init);");

  assert_eq!(Path::new("src/folder_structure.rs").exists(), true);

  assert_eq!(
    Path::new("../platformer/addons/directory_browser/folder_structure.gdns").exists(),
    true
  );

  set_current_dir("../").expect("Unable to change to parent directory");

  cleanup_test_files();

  Ok(())
}
