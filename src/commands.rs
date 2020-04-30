use std::env::{current_dir, set_current_dir};
use std::fs::read_to_string;
use std::fs::write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use crate::configs::{Cargo, Config, ConfigGeneral};
use crate::content;
use crate::utils;

/// Creates the library that will contain your Rust modules.
///
/// # Arguments
///
/// `destination` - The destination directory for the library.
/// `godot_project_dir` - The directory that contains the Godot project that the modules are for.
/// `targets` - The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
pub fn create_library(
    destination: std::path::PathBuf,
    godot_project_dir: std::path::PathBuf,
    targets: String,
) {
    info!("creating library");

    let dest_path = if !destination.is_absolute() {
        utils::absolute_path(destination).expect("Unable to create absolute path from destination path").as_path().to_owned()
    } else {
        Path::new(&destination).to_path_buf()
    };

    let godot_path = if !godot_project_dir.is_absolute() {
        utils::absolute_path(godot_project_dir).expect("Unable to create absolute path from destination path").as_path().to_owned()
    } else {
        Path::new(&godot_project_dir).to_path_buf()
    };

    // Check to see if the library already exists.
    if dest_path.exists() {
        error!("A library with the specified destination already exists, please choose another destination for the library.");
        exit(1);
    }

    // Check to see if the path to Godot project is valid.
    // In order for it to be considered valid, it needs to have a project.godot file.
    let godot_project_path = godot_path.join("project.godot");
    if !godot_project_path.exists() {
        error!("The godot project dir provided is not valid.");
        exit(1);
    }

    // Run the `cargo new` command in the destination directory to create the library.
    let dest_parent = dest_path
        .parent()
        .expect("Unable to get destination parent");
    let dest_basename = dest_path
        .file_stem()
        .expect("Unable to get destination basename");
    set_current_dir(&dest_parent).expect("Unable to change to destination parent directory");

    let dest_basename_string = dest_basename
        .to_str()
        .expect("Unable to convert destination basename to string");
    match Command::new("cargo")
        .arg("new")
        .arg(dest_basename_string)
        .arg("--lib")
        .output()
    {
        Ok(_v) => (),
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    }

    // Add the correct tags and dependencies to the library's Cargo.toml.
    set_current_dir(&dest_basename).expect("Unable to change to library directory");
    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA{:?}", current_dir());
    let cargo_toml_string = read_to_string("Cargo.toml").expect("Unable to resdfsdfsdfsdfsdfad Cargo.toml");
    let cargo_toml: Cargo = toml::from_str(&cargo_toml_string).expect("Unable to parse Cargo.toml");

    let cargo_toml_str = toml::to_string(&cargo_toml).expect("Unable to convert to toml to string");

    match write("Cargo.toml", cargo_toml_str.replace("\\", "")) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the library: {}", e);
            exit(1);
        }
    }

    // Check to see if the targets are valid.
    let valid_targets = &["windows", "linux", "osx"];
    let targets_split = targets.split(",").map(|s| s.to_string()).collect();

    for t in &targets_split {
        if !valid_targets.iter().any(|&i| i == t) {
            error!("An invalid target was specified: {}", t);
            exit(1);
        }
    }

    // Create the config and write it to a file.
    let config_general = ConfigGeneral {
        name: dest_basename_string.to_string(),
        path: dest_path.to_owned(),
        targets: targets_split,
        modules: vec![],
    };
    let config = Config {
        general: config_general,
    };

    let config_string = toml::to_string(&config).expect("Unable to convert config to string");

    match write("godot-rust-helper.toml", config_string) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the config file: {}", e);
            exit(1);
        }
    }

    // Create the initial src/lib.rs file for the library.
    match write("src/lib.rs", content::create_initial_lib_file()) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem writing to the lib file: {}", e);
            exit(1);
        }
    }

    // Create the 'rust-modules' directory in the Godot project so that we don't clutter the root directory.
    // let dir_to_change_to = current_dir().parent().parent();
    if !godot_path.is_absolute() {
        let current = current_dir().expect("Unable to get parent dir");
        let parent = current.as_path().parent().expect("Unable to get parent dir");
        let grandparent = parent.parent().expect("Unable to get grandparent dir");

        set_current_dir(&grandparent).expect("Unable to change to grandparent dir");
    }

    let godot_rust_modules_path = godot_path.join("rust-modules");
    match std::fs::create_dir_all(godot_rust_modules_path) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the the rust modules folder: {}", e);
            exit(1);
        }
    }

    // Create the gdnlib file for the library and save it to the rust-modules project directory.
    let targets_vec: Vec<String> = targets.split(",").map(|s| s.to_string()).collect();
    let targets_str: Vec<&str> = targets_vec.iter().map(AsRef::as_ref).collect();

    let gdnlib = content::create_gdnlib_file(dest_basename_string, &targets_str);
    let gdnlib_file_name = format!("{}.gdnlib", dest_basename_string);
    let gdnlib_path = godot_path.join("rust-modules").join(gdnlib_file_name);

    std::fs::File::create(&gdnlib_path).expect("Unable to create gdnlib file");

    match write(gdnlib_path, gdnlib.replace("\\", "")) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the gdnlib file: {}", e);
            exit(1);
        }
    }

    info!("library created successfully");
}

/// Creates a new module inside of the library.
///
/// # Arguments
///
/// `name` - The class name of the module to create; examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
pub fn create_module(name: &str) {
    info!("creating module");

    // Check to see if we are in the directory of a library created with the `new` command.
    // This is done by checking to see if there is a godot-rust-helper.toml configuration file present in the current directory.
    let current_dir_path =
        std::env::current_dir().expect("Unable to get current directory");
    let config_path = Path::new(&current_dir_path).join("godot-rust-helper.toml");
    if !config_path.exists() {
        error!(
            "The create command can only be used inside of a library created with the new command"
        );
        exit(1);
    }

    // Split the module name on capital letters and then make it all lowercase to use in some places.
    let name_normalized = utils::format_str(name.to_string());

    // Check the config to see if a module with the same name was already created.
    let config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");
    let mut config: Config = toml::from_str(&config_string).expect("Unable to parse config");

    if config.general.modules.iter().any(|i| i == name) {
        error!("A module with the same name already exists");
        exit(1);
    }

    // Save the module name to the config file so that it can be worked with later.
    config.general.modules.push(name.to_string());
    let new_config_string = toml::to_string(&config).expect("Unable to convert config to string");

    match write(config_path, new_config_string) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the module: {}", e);
            exit(1);
        }
    }

    // Create a new src/lib.rs file with the new module added to it.
    let lib_file = content::create_lib_file(&config.general.modules);
    let lib_file_path = current_dir_path.join("src").join("lib.rs");

    match write(lib_file_path, lib_file) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the lib file: {}", e);
            exit(1);
        }
    }

    // Create the default module file for the module.
    let mod_file = content::create_mod_file(&name);
    let mod_file_path = current_dir_path
        .join("src")
        .join(format!("{}.rs", name_normalized));

    std::fs::File::create(&mod_file_path).expect("Unable to create module file");

    match write(mod_file_path, mod_file) {
        Ok(_v) => (),
        Err(e) => {
            error!("There was a problem creating the module: {}", e);
            exit(1);
        }
    }

    info!("module created successfully");
}
