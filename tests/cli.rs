use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::{read_to_string, remove_dir_all, remove_file};
use std::path::Path;
use std::process::Command;

const PATH_TO_GODOT_RUST_HELPER: &str = "C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper";

// It should create a new library with the default Cargo.toml file.
#[test]
fn new_has_correct_cargo_toml() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    let cargo_toml =
        read_to_string("platformer_modules/Cargo.toml").expect("Unable to read Cargo.toml");
    let cargo_toml_split = cargo_toml.split("\n").collect::<Vec<&str>>();

    assert_eq!(cargo_toml_split[6], "[lib]");
    assert_eq!(cargo_toml_split[7], "crate-type = [\"cdylib\"]");
    assert_eq!(cargo_toml_split[9], "[dependencies]");
    assert_eq!(
        cargo_toml_split[10],
        "gdnative = \"0.9.1\""
    );
    assert_eq!(cargo_toml_split[11], "godot_rust_helper_ext = { git = \"https://github.com/robertcorponoi/godot_rust_helper_ext\" }");

    cleanup_after_test();

    Ok(())
}

// It should create a new library with the default targets set in the config.
#[test]
fn new_has_correct_default_config() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

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
    assert_eq!(config_split[7], format!("lib = \"{}\\\\tests\\\\platformer_modules\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[8], format!("godot = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[9], format!("output = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[10], format!("nativescript = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));

    assert_eq!(gdnlib_path.exists(), true);

    cleanup_after_test();

    Ok(())
}

// It should create a new library with a gdnlib containing the default targets.
#[test]
fn new_has_correct_gdnlib_default_targets() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

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

    cleanup_after_test();

    Ok(())
}

// It should create a library and place the gdnlib file in the specified output folder.
#[test]
fn new_specify_output_correct_gdnlib_location() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create a library and set a correct value for the nativescript output.
#[test]
fn new_specify_nativescript_correct_config() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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
    assert_eq!(config_split[7], format!("lib = \"{}\\\\tests\\\\platformer_modules\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[8], format!("godot = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[9], format!("output = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[10], format!("nativescript = \"{}\\\\tests\\\\platformer\\\\godot-rust-helper-scripts\"", PATH_TO_GODOT_RUST_HELPER));

    cleanup_after_test();

    Ok(())
}

// It should create a new library with --targets=windows,linux,osx and include them in the config.
#[test]
fn new_has_correct_targets_config() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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
    assert_eq!(config_split[7], format!("lib = \"{}\\\\tests\\\\platformer_modules\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[8], format!("godot = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[9], format!("output = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[10], format!("nativescript = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));

    cleanup_after_test();

    Ok(())
}

// It should create a new library with --targets=windows,linux-osx and include them in the gdnlib.
#[test]
fn new_has_correct_gdnlib_all_targets() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with the default Cargo.toml file.
#[test]
fn plugin_has_correct_cargo_toml() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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
    assert_eq!(
        cargo_toml_split[10],
        "gdnative = \"0.9.1\""
    );
    assert_eq!(cargo_toml_split[11], "godot_rust_helper_ext = { git = \"https://github.com/robertcorponoi/godot_rust_helper_ext\" }");

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with the default targets set in the config.
#[test]
fn plugin_has_correct_default_config() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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
    assert_eq!(config_split[7], format!("lib = \"{}\\\\tests\\\\directory_browser\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[8], format!("godot = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[9], format!("output = \"{}\\\\tests\\\\platformer\\\\addons\\\\directory_browser\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[10], format!("nativescript = \"{}\\\\tests\\\\platformer\\\\addons\\\\directory_browser\"", PATH_TO_GODOT_RUST_HELPER));

    assert_eq!(gdnlib_path.exists(), true);

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with a gdnlib containing the default targets.
#[test]
fn plugin_has_correct_gdnlib_default_targets() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with --targets=windows,linux,osx and include them in the config.
#[test]
fn plugin_has_correct_targets_config() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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
    assert_eq!(config_split[7], format!("lib = \"{}\\\\tests\\\\directory_browser\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[8], format!("godot = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[9], format!("output = \"{}\\\\tests\\\\platformer\\\\addons\\\\directory_browser\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[10], format!("nativescript = \"{}\\\\tests\\\\platformer\\\\addons\\\\directory_browser\"", PATH_TO_GODOT_RUST_HELPER));

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with --targets=windows,linux-osx and include them in the gdnlib.
#[test]
fn plugin_has_correct_gdnlib_all_targets() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with the correct plugin.cfg file.
#[test]
fn plugin_has_correct_default_plugin_cfg_file() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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
    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with the correct plugin.cfg file.
#[test]
fn plugin_has_correct_plugin_cfg_file() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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
    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with the correct plugin base mod file.
#[test]
fn plugin_has_correct_plugin_base_file() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
        .arg("Directory Browser")
        .arg("directory_browser")
        .arg("platformer")
        .arg("--description")
        .arg("A test plugin")
        .arg("--author")
        .arg("Bob")
        .arg("--targets=windows,linux,osx");

    cmd.assert().success();

    let plugin_base_gdns = read_to_string("directory_browser/src/directory_browser.rs")
        .expect("Unable to read mod file");
    let plugin_base_gdns_split = plugin_base_gdns.split("\n").collect::<Vec<&str>>();

    assert_eq!(plugin_base_gdns_split[0], "use gdnative::api::EditorPlugin;");
    assert_eq!(plugin_base_gdns_split[1], "use gdnative::nativescript::user_data;");
    assert_eq!(plugin_base_gdns_split[2], "");
    assert_eq!(
        plugin_base_gdns_split[3],
        "#[derive(NativeClass)]"
    );
    assert_eq!(
        plugin_base_gdns_split[4],
        "#[inherit(EditorPlugin)]"
    );
    assert_eq!(plugin_base_gdns_split[5], "#[user_data(user_data::LocalCellData<DirectoryBrowser>)]");
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

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with the correct plugin gdns file.
#[test]
fn plugin_has_correct_plugin_gdns() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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

    cleanup_after_test();

    Ok(())
}

// It should create a new plugin with with the correct src/lib.rs file.
#[test]
fn plugin_has_correct_lib_file() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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

    cleanup_after_test();

    Ok(())
}

// It should create a module and add an entry for it in the config file.
#[test]
fn create_add_module_to_config() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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
    assert_eq!(config_split[7], format!("lib = \"{}\\\\tests\\\\platformer_modules\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[8], format!("godot = \"{}\\\\tests\\\\platformer\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[9], format!("output = \"{}\\\\tests\\\\platformer\\\\godot-rust-helper-output\"", PATH_TO_GODOT_RUST_HELPER));
    assert_eq!(config_split[10], format!("nativescript = \"{}\\\\tests\\\\platformer\\\\godot-rust-helper-scripts\"", PATH_TO_GODOT_RUST_HELPER));

    set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create a module and add it to the lib file.
#[test]
fn create_add_module_to_lib() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create a module and add create a module file for it.
#[test]
fn create_mod_file() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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
    assert_eq!(mod_file_split[5], "#[user_data(user_data::LocalCellData<Hello>)]");
    assert_eq!(mod_file_split[6], "pub struct Hello;");
    assert_eq!(mod_file_split[7], "");
    assert_eq!(mod_file_split[8], "#[gdnative::methods]");
    assert_eq!(mod_file_split[9], "impl Hello {");
    assert_eq!(
        mod_file_split[10],
        "\tfn new(_owner: &Node) -> Self {"
    );
    assert_eq!(mod_file_split[11], "\t\tHello");
    assert_eq!(mod_file_split[12], "\t}");
    assert_eq!(mod_file_split[13], "");
    assert_eq!(mod_file_split[14], "\t#[export]");
    assert_eq!(
        mod_file_split[15],
        "\tfn _ready(&self, _owner: &Node) {"
    );
    assert_eq!(mod_file_split[16], "\t\tgodot_print!(\"hello, world.\");");
    assert_eq!(mod_file_split[17], "\t}");
    assert_eq!(mod_file_split[18], "}");
    assert_eq!(mod_file_split[19], "");

    set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create a module and add create a gdns file for it.
#[test]
fn create_gdns_file_at_default_location() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create a module and add create a gdns file for it at the specified nativescript path.
#[test]
fn create_gdns_file_at_specified_nativescript_location() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create a with multiple capital letters in the name.
#[test]
fn create_multiple_captial_letters() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create multiple modules.
#[test]
fn create_multiple_modules() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create multiple modules and add them to the lib file.
#[test]
fn create_multiple_modules_and_add_to_lib() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create a module inside of a plugin.
#[test]
fn create_additional_plugin_script() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("plugin")
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

    cleanup_after_test();

    Ok(())
}

// It should remove all traces of a created module.
#[test]
fn destroy_remove_created_module() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create two modules and remove one.
#[test]
fn destroy_create_two_remove_one() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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

    cleanup_after_test();

    Ok(())
}

// It should create two modules and remove one.
#[test]
fn build_library() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
        .arg("platformer_modules")
        .arg("platformer")
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
        .arg("build")
        .output()
        .expect("Unable to execute cargo run");

    let dll_file_path = Path::new("../platformer/platformer_modules.dll");

    assert_eq!(dll_file_path.exists(), true);

    set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should place the build files in the correct specified output.
#[test]
fn build_specify_output_correct_dll_location() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
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
        .arg("build")
        .output()
        .expect("Unable to execute cargo run");

    let dll_file_path = Path::new("../platformer/godot-rust-helper-output/platformer_modules.dll");

    assert_eq!(dll_file_path.exists(), true);

    set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// Makes sure that the test are running from the correct directory.
fn ensure_correct_dir() {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_basename = current_dir.file_stem().unwrap();

    if current_dir_basename != "tests" {
        set_current_dir("tests").expect("Unable to change to tests directory");
    }
}

// Removes the platformer_modules folder and the gdnlib/dll files.
fn cleanup_after_test() {
    if Path::new("platformer_modules").exists() {
        remove_dir_all("platformer_modules").expect("Unable to remove dir");
    }

    if Path::new("directory_browser").exists() {
        remove_dir_all("directory_browser").expect("Unable to remove dir");
    }

    if Path::new("platformer/platformer_modules.gdnlib").exists() {
        remove_file("platformer/platformer_modules.gdnlib").expect("Unable to remove file");
        if Path::new("platformer/platformer_modules.dll").exists() {
            remove_file("platformer/platformer_modules.dll").expect("Unable to remove file");
        }
    } else if Path::new("platformer/godot-rust-helper-output").exists() {
        remove_dir_all("platformer/godot-rust-helper-output").expect("Unable to remove dir")
    }

    if Path::new("platformer/godot-rust-helper-scripts").exists() {
        remove_dir_all("platformer/godot-rust-helper-scripts").expect("Unable to remove dir");
    }

    if Path::new("platformer/addons").exists() {
        remove_dir_all("platformer/addons").expect("Unable to remove dir");
    }
}
