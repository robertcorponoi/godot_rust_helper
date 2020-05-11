use assert_cmd::prelude::*;

use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

// It should create a new library with the default Cargo.toml file.
#[test]
fn new_has_correct_cargo_toml() -> Result<(), Box<dyn std::error::Error>> {
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
        "gdnative = { git = \"https://github.com/GodotNativeTools/godot-rust\" }"
    );
    assert_eq!(cargo_toml_split[11], "godot_rust_helper_ext = { git = \"https://github.com/robertcorponoi/godot_rust_helper_ext\" }");

    cleanup_after_test();

    Ok(())
}

// It should create a new library with the default targets set in the config.
#[test]
fn new_has_correct_default_config() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    let config =
        read_to_string("platformer_modules/godot-rust-helper.toml").expect("Unable to read config");
    let config_split = config.split("\n").collect::<Vec<&str>>();

    assert_eq!(config_split[0], "[general]");
    assert_eq!(config_split[1], "name = \"platformer_modules\"");
    assert_eq!(config_split[2], "lib_path = \"C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper\\\\tests\\\\platformer_modules\"");
    assert_eq!(config_split[3], "godot_path = \"C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper\\\\tests\\\\platformer\"");
    assert_eq!(config_split[4], "targets = [\"windows\"]");
    assert_eq!(config_split[5], "modules = []");

    cleanup_after_test();

    Ok(())
}

// It should create a new library with a gdnlib containing the default targets.
#[test]
fn new_has_correct_gdnlib_default_targets() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    let gdnlib = read_to_string("platformer/rust-modules/platformer_modules.gdnlib")
        .expect("Unable to read gdnlib");
    let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

    assert_eq!(gdnlib_split[0], "[entry]");
    assert_eq!(gdnlib_split[1], "");
    assert_eq!(
        gdnlib_split[2],
        "Windows.64=\"res://rust-modules/platformer_modules.dll\""
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

// It should create a new library with --targets=windows,linux,osx and include them in the config.
#[test]
fn new_has_correct_targets_config() -> Result<(), Box<dyn std::error::Error>> {
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
    assert_eq!(config_split[2], "lib_path = \"C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper\\\\tests\\\\platformer_modules\"");
    assert_eq!(config_split[3], "godot_path = \"C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper\\\\tests\\\\platformer\"");
    assert_eq!(
        config_split[4],
        "targets = [\"windows\", \"linux\", \"osx\"]"
    );
    assert_eq!(config_split[5], "modules = []");

    cleanup_after_test();

    Ok(())
}

// It should create a new library with --targets=windows,linux-osx and include them in the gdnlib.
#[test]
fn new_has_correct_gdnlib_all_targets() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
        .arg("platformer_modules")
        .arg("platformer")
        .arg("--targets=windows,linux,osx");

    cmd.assert().success();

    let gdnlib = read_to_string("platformer/rust-modules/platformer_modules.gdnlib")
        .expect("Unable to read gdnlib");
    let gdnlib_split = gdnlib.split("\n").collect::<Vec<&str>>();

    assert_eq!(gdnlib_split[0], "[entry]");
    assert_eq!(gdnlib_split[1], "");
    assert_eq!(
        gdnlib_split[2],
        "OSX.64=\"res://rust-modules/libplatformer_modules.dylib\""
    );
    assert_eq!(
        gdnlib_split[3],
        "X11.64=\"res://rust-modules/libplatformer_modules.so\""
    );
    assert_eq!(
        gdnlib_split[4],
        "Windows.64=\"res://rust-modules/platformer_modules.dll\""
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

// It should create a module and add an entry for it in the config file.
#[test]
fn create_add_module_to_config() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
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
    assert_eq!(config_split[2], "lib_path = \"C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper\\\\tests\\\\platformer_modules\"");
    assert_eq!(config_split[3], "godot_path = \"C:\\\\Users\\\\Bob\\\\Documents\\\\Projects\\\\godot_rust_helper\\\\tests\\\\platformer\"");
    assert_eq!(config_split[4], "targets = [\"windows\"]");
    assert_eq!(config_split[5], "modules = [\"Hello\"]");

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create a module and add it to the lib file.
#[test]
fn create_add_module_to_lib() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
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
        "fn init(handle: gdnative::init::InitHandle) {"
    );
    assert_eq!(lib_file_split[6], "\thandle.add_class::<hello::Hello>();");
    assert_eq!(lib_file_split[7], "}");
    assert_eq!(lib_file_split[8], "");
    assert_eq!(lib_file_split[9], "godot_gdnative_init!();");
    assert_eq!(lib_file_split[10], "godot_nativescript_init!(init);");
    assert_eq!(lib_file_split[11], "godot_gdnative_terminate!();");

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create a module and add create a module file for it.
#[test]
fn create_mod_file() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Hello")
        .output()
        .expect("Unable to execute cargo run");

    let mod_file = read_to_string("src/hello.rs").expect("Unable to read module file");
    let mod_file_split = mod_file.split("\n").collect::<Vec<&str>>();

    assert_eq!(mod_file_split[0], "#[derive(gdnative::NativeClass)]");
    assert_eq!(mod_file_split[1], "#[inherit(gdnative::Node)]");
    assert_eq!(mod_file_split[2], "pub struct Hello;");
    assert_eq!(mod_file_split[3], "");
    assert_eq!(mod_file_split[4], "#[gdnative::methods]");
    assert_eq!(mod_file_split[5], "impl Hello {");
    assert_eq!(
        mod_file_split[6],
        "\tfn _init(_owner: gdnative::Node) -> Self {"
    );
    assert_eq!(mod_file_split[7], "\t\tHello");
    assert_eq!(mod_file_split[8], "\t}");
    assert_eq!(mod_file_split[9], "");
    assert_eq!(mod_file_split[10], "\t#[export]");
    assert_eq!(
        mod_file_split[11],
        "\tfn _ready(&self, _owner: gdnative::Node) {"
    );
    assert_eq!(mod_file_split[12], "\t\tgodot_print!(\"hello, world.\");");
    assert_eq!(mod_file_split[13], "\t}");
    assert_eq!(mod_file_split[14], "}");
    assert_eq!(mod_file_split[15], "");

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create a with multiple capital letters in the name.
#[test]
fn create_multiple_captial_letters() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
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

    assert_eq!(mod_file_split[2], "pub struct MainScene;");
    assert_eq!(mod_file_split[5], "impl MainScene {");
    assert_eq!(mod_file_split[7], "\t\tMainScene");

    assert_eq!(config_split[5], "modules = [\"MainScene\"]");

    assert_eq!(lib_file_split[3], "mod main_scene;");
    assert_eq!(
        lib_file_split[6],
        "\thandle.add_class::<main_scene::MainScene>();"
    );

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create multiple modules.
#[test]
fn create_multiple_modules() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Hello")
        .output()
        .expect("Unable to execute cargo run");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("World")
        .output()
        .expect("Unable to execute cargo run");

    let hello_file_path = std::path::Path::new("src/hello.rs");
    let world_file_path = std::path::Path::new("src/lib.rs");
    let gdnlib_file_path =
        std::path::Path::new("../platformer/rust-modules/platformer_modules.gdnlib");

    let config_file = read_to_string("godot-rust-helper.toml").expect("Unable to read config file");
    let config_split = config_file.split("\n").collect::<Vec<&str>>();

    assert_eq!(hello_file_path.exists(), true);
    assert_eq!(world_file_path.exists(), true);
    assert_eq!(gdnlib_file_path.exists(), true);
    assert_eq!(config_split[5], "modules = [\"Hello\", \"World\"]");

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create multiple modules and add them to the lib file.
#[test]
fn create_multiple_modules_and_add_to_lib() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Hello")
        .output()
        .expect("Unable to execute cargo run");
    std::process::Command::new("cargo")
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
        "fn init(handle: gdnative::init::InitHandle) {"
    );
    assert_eq!(lib_file_split[7], "\thandle.add_class::<hello::Hello>();");
    assert_eq!(lib_file_split[8], "\thandle.add_class::<world::World>();");
    assert_eq!(lib_file_split[9], "}");
    assert_eq!(lib_file_split[10], "");
    assert_eq!(lib_file_split[11], "godot_gdnative_init!();");
    assert_eq!(lib_file_split[12], "godot_nativescript_init!(init);");
    assert_eq!(lib_file_split[13], "godot_gdnative_terminate!();");

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should remove all traces of a created module.
#[test]
fn destroy_remove_created_module() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Hello")
        .output()
        .expect("Unable to execute cargo run");
    std::process::Command::new("cargo")
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

    assert_eq!(lib_file_split[0], "#[macro_use]");
    assert_eq!(lib_file_split[1], "extern crate gdnative;");
    assert_eq!(lib_file_split[2], "");
    assert_eq!(
        lib_file_split[3],
        "fn init(handle: gdnative::init::InitHandle) {"
    );
    assert_eq!(lib_file_split[4], "}");
    assert_eq!(lib_file_split[5], "");
    assert_eq!(lib_file_split[6], "godot_gdnative_init!();");
    assert_eq!(lib_file_split[7], "godot_nativescript_init!(init);");
    assert_eq!(lib_file_split[8], "godot_gdnative_terminate!();");

    assert_eq!(config_split[5], "modules = []");

    assert_eq!(mod_file_path.exists(), false);

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create two modules and remove one.
#[test]
fn destory_create_two_remove_one() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Hello")
        .output()
        .expect("Unable to execute cargo run");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("World")
        .output()
        .expect("Unable to execute cargo run");
    std::process::Command::new("cargo")
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

    assert_eq!(lib_file_split[0], "#[macro_use]");
    assert_eq!(lib_file_split[1], "extern crate gdnative;");
    assert_eq!(lib_file_split[2], "");
    assert_eq!(lib_file_split[3], "mod hello;");
    assert_eq!(lib_file_split[4], "");
    assert_eq!(
        lib_file_split[5],
        "fn init(handle: gdnative::init::InitHandle) {"
    );
    assert_eq!(lib_file_split[6], "\thandle.add_class::<hello::Hello>();");
    assert_eq!(lib_file_split[7], "}");
    assert_eq!(lib_file_split[8], "");
    assert_eq!(lib_file_split[9], "godot_gdnative_init!();");
    assert_eq!(lib_file_split[10], "godot_nativescript_init!(init);");
    assert_eq!(lib_file_split[11], "godot_gdnative_terminate!();");

    assert_eq!(config_split[5], "modules = [\"Hello\"]");

    assert_eq!(hello_mod_file_path.exists(), true);
    assert_eq!(world_mod_file_path.exists(), false);

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// It should create two modules and remove one.
#[test]
fn build_library() -> Result<(), Box<dyn std::error::Error>> {
    ensure_correct_dir();

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new").arg("platformer_modules").arg("platformer");

    cmd.assert().success();

    std::env::set_current_dir("platformer_modules").expect("Unable to change to library directory");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("create")
        .arg("Hello")
        .output()
        .expect("Unable to execute cargo run");
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("build")
        .output()
        .expect("Unable to execute cargo run");

    let dll_file_path = Path::new("../platformer/rust-modules/platformer_modules.dll");

    assert_eq!(dll_file_path.exists(), true);

    std::env::set_current_dir("../").expect("Unable to change to parent directory");

    cleanup_after_test();

    Ok(())
}

// Makes sure that the test are running from the correct directory.
fn ensure_correct_dir() {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_basename = current_dir.file_stem().unwrap();

    if current_dir_basename != "tests" {
        std::env::set_current_dir("tests").expect("Unable to change to tests directory");
    }
}

// Removes the platformer_modules and platformer/rust-modules folders.
fn cleanup_after_test() {
    std::fs::remove_dir_all("platformer_modules").expect("Unable to remove dir");
    std::fs::remove_dir_all("platformer/rust-modules").expect("Unable to remove dir");
}
