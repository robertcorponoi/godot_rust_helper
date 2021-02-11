use std::env::{current_dir, set_current_dir};
use std::fs::{read_to_string, remove_file, write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::mpsc::channel;

use crate::configs::{
    Cargo, Config, ConfigGeneral, ConfigGeneralV3, ConfigPaths, ConfigPathsV2, ConfigV1, ConfigV2,
    ConfigV3, PluginConfig, PluginConfigFields,
};
use crate::content;
use crate::utils;

use chrono::prelude::*;
use colored::*;
use convert_case::{Case, Casing};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use pathdiff::diff_paths;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Runs `cargo new --lib` to create the library that will contain your Rust components.
///
/// # Arguments
///
/// `destination` - The destination directory for the library.
/// `godot_project_dir` - The directory that contains the Godot project that the modules are for.
/// `targets` - The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
/// `output` - A directory within the godot project to place the gdnlib and build files.
/// `nativescript_path` - A directory within the godot project to place the nativescript files.
pub fn create_library(
    destination: PathBuf,
    godot_project_dir: PathBuf,
    targets: String,
    output: PathBuf,
    nativescript_path: PathBuf,
) {
    println!("{}", "creating library".white());

    // Make the destination directory is an absolute path if it is not already one.
    let dest_path = if !destination.is_absolute() {
        utils::absolute_path(destination)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&destination).to_path_buf()
    };

    // Make the godot directory is an absolute path if it is not already one.
    let godot_path = if !godot_project_dir.is_absolute() {
        utils::absolute_path(godot_project_dir)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&godot_project_dir).to_path_buf()
    };

    // Make the output path is an absolute path if it is not already one.
    let output_as_path_buf = PathBuf::from(output);
    let output_path = if output_as_path_buf == PathBuf::from("") {
        PathBuf::from(&godot_path)
    } else if !output_as_path_buf.is_absolute() {
        utils::absolute_path(output_as_path_buf)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&output_as_path_buf).to_path_buf()
    };

    // Make the nativescript path an absolute path if it is not already one.
    let ns_path_as_path_buf = PathBuf::from(nativescript_path);
    let ns_path = if ns_path_as_path_buf == PathBuf::from("") {
        PathBuf::from(&godot_path)
    } else if !ns_path_as_path_buf.is_absolute() {
        utils::absolute_path(ns_path_as_path_buf)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&ns_path_as_path_buf).to_path_buf()
    };

    // Check to see if the destination directory already exists, we don't want to overwrite an existing project.
    if dest_path.exists() {
        println!("A library with the specified destination already exists, please choose another destination for the library.");
        exit(1);
    }

    // Check to make sure that the provided path to the Godot project is valid, i.e. it has a project.godot file at its root.
    let godot_project_path = godot_path.join("project.godot");
    if !godot_project_path.exists() {
        println!("The godot project dir provided is not valid.");
        exit(1);
    }

    // Run the `cargo new --lib` command in the destination directory to create the library.
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
            println!("{}", e);
            exit(1);
        }
    }

    // Set up the Cargo.toml file of the library to have the required tags and dependencies.
    set_current_dir(&dest_basename).expect("Unable to change to library directory");
    let cargo_toml_string = read_to_string("Cargo.toml").expect("Unable to read Cargo.toml");
    let cargo_toml: Cargo = toml::from_str(&cargo_toml_string).expect("Unable to parse Cargo.toml");
    let cargo_toml_str = toml::to_string(&cargo_toml).expect("Unable to convert to toml to string");

    match write(
        "Cargo.toml",
        cargo_toml_str
            .replace("\\", "")
            .replace("\"{", "{")
            .replace("}\"", "}"),
    ) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the library: {}", e);
            exit(1);
        }
    }

    // Make sure the targets provided are in the list of accepted targets.
    let valid_targets = &["windows", "linux", "osx"];
    let targets_split = targets.split(",").map(|s| s.to_string()).collect();
    for t in &targets_split {
        if !valid_targets.iter().any(|&i| i == t) {
            println!("An invalid target was specified: {}", t);
            exit(1);
        }
    }

    // Create the config and write it to a godot-rust-helper.toml file.
    let config_paths = ConfigPaths {
        lib: dest_path.to_owned(),
        godot: godot_path.to_owned(),
        output: output_path.to_owned(),
        nativescript: ns_path.to_owned(),
    };
    let config_general = ConfigGeneral {
        name: dest_basename_string.to_string(),
        modules: vec![],
        targets: targets_split,
        plugin: false,
    };
    let config = Config {
        general: config_general,
        paths: config_paths,
    };
    let config_string = toml::to_string(&config).expect("Unable to convert config to string");

    match write("godot-rust-helper.toml", config_string) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the config file: {}", e);
            exit(1);
        }
    }

    // Create the initial src/lib.rs file for the library that will contain all of the created modules.
    match write("src/lib.rs", content::create_initial_lib_file()) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem writing to the lib file: {}", e);
            exit(1);
        }
    }

    // Create the directory structure specified by --output-path.
    match std::fs::create_dir_all(&config.paths.output) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "There was a problem creating the the output directory structure: {}",
                e
            );
            exit(1);
        }
    }

    // Create the directory structure specified by --nativescript-path.
    match std::fs::create_dir_all(&config.paths.nativescript) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "There was a problem creating the the nativescript directory structure: {}",
                e
            );
            exit(1);
        }
    }

    // Format the targets so that we can use them to create the gndlib file.
    let targets_vec: Vec<String> = targets.split(",").map(|s| s.to_string()).collect();
    let targets_str: Vec<&str> = targets_vec.iter().map(AsRef::as_ref).collect();

    // Create the contents of the gdnlib file and write it to the output directory.
    let gdnlib = content::create_gdnlib_file(
        dest_basename_string,
        &diff_paths(&output_path, &godot_path).expect("Unable to get output path diff"),
        &targets_str,
    );
    let gdnlib_file_name = format!("{}.gdnlib", dest_basename_string);

    std::fs::File::create(&config.paths.output.join(&gdnlib_file_name))
        .expect("Unable to create gdnlib file");

    match write(config.paths.output.join(gdnlib_file_name), gdnlib) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the gdnlib file: {}", e);
            exit(1);
        }
    }

    println!("{}", "library created".green());
}

/// Creates a new module inside of the library.
///
/// # Arguments
///
/// `name` - The class name of the module to create; examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
pub fn create_module(name: &str) {
    println!("{}", "creating module".white());

    // Check to make sure we are in a godot_rust_helper project by checking for the presence of a godot-rust-helper.toml file.
    let current_dir_path = std::env::current_dir().expect("Unable to get current directory");
    let config_path = Path::new(&current_dir_path).join("godot-rust-helper.toml");
    if !config_path.exists() {
        println!(
            "The create command can only be used inside of a library created with the new command"
        );
        exit(1);
    }

    // Split the module name on capital letters and then make it all lowercase to use in some places.
    let name_normalized = name.to_case(Case::Snake);

    // Check the config to see if a module with the same name was already created.
    let config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");
    let mut config: Config = toml::from_str(&config_string).expect("Unable to parse config");

    if config.general.modules.iter().any(|i| i == name) {
        println!("A module with the same name already exists");
        exit(1);
    }

    // Save the module name to the config file so that it can be worked with later.
    config.general.modules.push(name.to_string());
    let new_config_string = toml::to_string(&config).expect("Unable to convert config to string");

    match write(config_path, new_config_string) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the module: {}", e);
            exit(1);
        }
    }

    // Create a new src/lib.rs file with the new module added to it.
    let lib_file = content::create_lib_file(&config.general.modules, config.general.plugin);
    let lib_file_path = current_dir_path.join("src").join("lib.rs");

    match write(lib_file_path, lib_file) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the lib file: {}", e);
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
            println!("There was a problem creating the mod file: {}", e);
            exit(1);
        }
    }

    // Create the gdns file.
    let gdns_file_contents = content::create_gdns_file(
        &config.general.name,
        &name,
        &diff_paths(&config.paths.output, &config.paths.godot)
            .expect("Unable to get output path diff"),
    );
    let gdns_file_name = format!("{}.gdns", name_normalized);
    let gdns_file_path = &config.paths.nativescript.join(gdns_file_name);

    std::fs::File::create(&gdns_file_path).expect("Unable to create gdns file");

    match write(gdns_file_path, gdns_file_contents) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the gdns file: {}", e);
            exit(1);
        }
    }

    println!("{}", "module created".green());
}

/// Removes a created module from the config and filesystem.
///
/// # Arguments
///
/// `name` The name of the module to destroy. This should be the same name that was provided when it was created.
pub fn destroy_module(name: &str) {
    println!("{}", "destroying module".white());

    // Check to see if we are in the directory of a library created with the `new` command by checking for the presence of a godot-rust-helper.toml file.
    let current_dir_path = std::env::current_dir().expect("Unable to get current directory");
    let config_path = Path::new(&current_dir_path).join("godot-rust-helper.toml");
    if !config_path.exists() {
        println!(
            "The create command can only be used inside of a library created with the new command"
        );
        exit(1);
    }

    // Split the module name on capital letters and then make it all lowercase to use when removing the module file.
    let name_normalized = name.to_case(Case::Snake);

    // Remove the module from the config file and save it again.
    let config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");
    let mut config: Config = toml::from_str(&config_string).expect("Unable to parse config");

    config.general.modules.retain(|x| *x != name);
    let new_config_string = toml::to_string(&config).expect("Unable to convert config to string");

    match write(config_path, new_config_string) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem destroying the module: {}", e);
            exit(1);
        }
    }

    // Create a new src/lib.rs file based on the modules that are left over.
    let lib_file = if config.general.modules.len() == 0 {
        content::create_initial_lib_file()
    } else {
        content::create_lib_file(&config.general.modules, config.general.plugin)
    };

    let lib_file_path = current_dir_path.join("src").join("lib.rs");

    match write(lib_file_path, lib_file) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the lib file: {}", e);
            exit(1);
        }
    }

    // Remove the module's .rs file.
    let mod_file_path = current_dir_path
        .join("src")
        .join(format!("{}.rs", name_normalized));
    match remove_file(mod_file_path) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem removing the module file: {}", e);
            exit(1);
        }
    }

    // Remove the corresponding .gdns file from the Godot project directory.
    let gdns_file_name = format!("{}.gdns", name.to_lowercase());
    Command::new("rm")
        .arg(&config.paths.nativescript.join(gdns_file_name))
        .status()
        .expect("Unable to remove the corresponding gdns file from the Godot project.");

    println!("{}", "module destroyed".green());
}

/// Runs the `cargo build` command and copies the target files into the Godot project directory.
pub fn build_library() {
    let version_notice = format!(
        "{}{}",
        "godot_rust_helper v".white().underline(),
        VERSION.white().underline()
    );
    println!("{}", version_notice);
    println!("{}", "building...".cyan());

    let root_dir = utils::find_file("godot-rust-helper.toml".to_string());

    // Get the config so that we can check the targets later.
    let config_path = root_dir.join("godot-rust-helper.toml");
    let config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");
    let config: Config = toml::from_str(&config_string).expect("Unable to parse config");

    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Unable to run cargo build");

    if !status.success() {
        println!("{}", "build failed".red());
        exit(1);
    }

    // Get the path to where the build files are stored.
    let targets_dir = root_dir.join("target").join("debug");

    let ext = utils::get_dynamic_library_ext();
    let extra = if cfg!(windows) { "" } else { "lib" };

    let file = format!("{}{}.{}", extra, config.general.name, ext);
    let file_path = targets_dir.join(file.to_case(Case::Snake));

    Command::new("cp")
        .arg(file_path)
        .arg(&config.paths.godot.join(&config.paths.output))
        .output()
        .expect("Unable to copy build files");

    println!("{}", "build complete".green());

    // Delete this next iteration if not needed anymore.
    // for target in &config.general.targets {
    //     // Since the user can have multiple targets, we need to check what target
    //     // they can actually build for and set that as the target to use.
    //     match target.as_ref() {
    //         "linux" => {
    //         },
    //         _ => exit(1)
    //     }

    //     let ext = if target == "linux" {
    //         "so"
    //     } else if target == "osx" {
    //         "dylib"
    //     } else if target == "windows" {
    //         "dll"
    //     } else {
    //         exit(1);
    //     };
    //     let extra = if cfg!(windows) { "" } else { "lib" };

    //     let file = format!("{}{}.{}", extra, config.general.name, ext);
    //     let file_path = targets_dir.join(file);

    //     Command::new("cp")
    //         .arg(file_path)
    //         .arg(&config.paths.godot.join(&config.paths.output))
    //         .output()
    //         .expect("Unable to copy build files");

    //     println!("{}", "build complete".green());
    // }
}

/// Watches for changes in the src directory of the library and then automatically runs the build command.
/// This is used when build --watch is called.
pub fn watch_library() {
    let lib_dir = utils::find_file("godot-rust-helper.toml".to_string());
    let (tx, rx) = channel();

    build_with_time_log();

    let mut last_checked = chrono::offset::Local::now();

    let mut watcher = raw_watcher(tx).expect("Unable to create watcher");
    watcher
        .watch(lib_dir.join("src"), RecursiveMode::Recursive)
        .expect("Unable to watch src directory");

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(_path),
                op: Ok(op),
                cookie: _,
            }) => {
                if op.contains(op::WRITE) {
                    let now = chrono::offset::Local::now();
                    if (now - last_checked).num_seconds() == 0 {
                        build_with_time_log();
                    }
                    last_checked = chrono::offset::Local::now();
                }
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

/// Changes the path of the project and the godot project directory in the config if you cloned/downlaoded the project from elsewhere.
///
/// # Arguments
///
/// `godot_project_dir` - The directory that contains the Godot project that the modules are for.
/// `targets` - The new build targets that should be set.
pub fn rebase(godot_project_dir: PathBuf, targets: String) {
    println!("{}", "rebasing library".white());

    // Check to see if we are in the directory of a library created with the `new` command.
    // This is done by checking to see if there is a godot-rust-helper.toml configuration file present in the current directory.
    let current_dir_path = std::env::current_dir().expect("Unable to get current directory");
    let config_path = Path::new(&current_dir_path).join("godot-rust-helper.toml");
    if !config_path.exists() {
        println!(
            "The create command can only be used inside of a library created with the new command"
        );
        exit(1);
    }

    // Get the config file so that we can update values.
    let config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");
    let mut config: Config = toml::from_str(&config_string).expect("Unable to parse config");

    // Get the new paths to use for the config.
    let lib_path = current_dir().expect("Unable to get current directory");
    let godot_path = utils::absolute_path(godot_project_dir)
        .expect("Unable to create absolute path from destination path")
        .as_path()
        .to_owned();

    // Update the paths in the config.
    config.paths.lib = lib_path;
    config.paths.godot = godot_path;

    // If new targets were set then we update it in the config and the gdnlib file.
    if !targets.is_empty() {
        let valid_targets = &["windows", "linux", "osx"];
        let targets_split: Vec<String> = targets.split(",").map(|s| s.to_string()).collect();

        for t in &targets_split {
            if !valid_targets.iter().any(|&i| i == t) {
                println!("An invalid target was specified: {}", t);
                exit(1);
            }
        }

        config.general.targets = targets_split;
    }

    let targets_vec: Vec<String> = targets.split(",").map(|s| s.to_string()).collect();
    let targets_str: Vec<&str> = targets_vec.iter().map(AsRef::as_ref).collect();

    let gdnlib = content::create_gdnlib_file(
        &config.general.name.to_owned(),
        &config.paths.output,
        &targets_str,
    );
    let gdnlib_file_name = format!("{}.gdnlib", &config.general.name.to_owned());
    let gdnlib_path = config.paths.output.join(gdnlib_file_name);

    match write(gdnlib_path, gdnlib.replace("\\", "")) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the gdnlib file: {}", e);
            exit(1);
        }
    }

    // Finally we write back the config file.
    let config_string = toml::to_string(&config).expect("Unable to convert config to string");
    match write("godot-rust-helper.toml", config_string) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the config file: {}", e);
            exit(1);
        }
    }

    println!("{}", "library rebased".green());
}

/// Updates a library from an older version of godot_rust_helper to the newest version.
///
/// # Arguments
///
/// `output` - As of godot_rust_helper 2.x the 'rust-modules' directory no longer exists and is customizable. You can change this to a different directory at this time but you'll have to fix all references in Godot.
/// `nativescript_path` - As of godot_rust_helper 3.x the nativescript files can be placed into a custom directory within the Godot project.
pub fn update(output: PathBuf, nativescript_path: PathBuf) {
    println!(
        "{}",
        "Updating project from an older version of godot_rust_helper...".white()
    );

    // Check to see if we are in a library created by the `new` command.
    let current_dir_path = current_dir().expect("Unable to get current directory");
    let config_path = current_dir_path.join("godot-rust-helper.toml");
    if !config_path.exists() {
        println!(
            "The upgrade command can only be used inside of a library created with the new command"
        );
        exit(1);
    }

    // Read and parse the old config file.
    let mut config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");

    // Check to see what version the current project is using by looking for key differences in the config file.
    // If the config file doesn't have the newer config format with a [paths] section, then it is v1.x. If the config file doesn't have a nativescript_path entry, then it is v2.x.
    // If the config file has the [paths] section but doesn't have the nativescript entry under it, then the project is v2.x.
    if !config_string.contains("[paths]") {
        // Parse the current configuration of the project of the project as configuration v1.
        let current_config: ConfigV1 = toml::from_str(&config_string).expect(
            "Unable to parse godot-rust-helper config file of project using godot_rust_helper v1.x",
        );

        // Create an instance of the new configuration of the project using the values from the current config.
        // Since absolute paths were not a thing until godot_rust_helper v2 we also have to create absolute paths from the old paths.
        let new_config_paths = ConfigPathsV2 {
            lib: utils::absolute_path(current_config.general.lib_path)
                .expect("Unable to create absolute path from godot_rust_helper v1.x lib path"),
            godot: utils::absolute_path(current_config.general.godot_path).expect(
                "Unable to create absolute path from from godot_rust_helper v1.x godot path",
            ),
            output: PathBuf::new(),
        };
        let new_config_general = ConfigGeneral {
            name: current_config.general.name,
            targets: current_config.general.targets,
            modules: current_config.general.modules,
            plugin: false,
        };
        let mut new_config = ConfigV2 {
            general: new_config_general,
            paths: new_config_paths,
        };

        // Check to see if the user would like to get rid of the rust-modules folder and instead use another directory within the Godot project.
        let output_as_path_buf = PathBuf::from(output);
        let output_path = if output_as_path_buf == PathBuf::from("") {
            PathBuf::from(&new_config.paths.godot.join("rust-modules"))
        } else if !output_as_path_buf.is_absolute() {
            utils::absolute_path(output_as_path_buf)
                .expect("Unable to create absolute path from destination path")
                .as_path()
                .to_owned()
        } else {
            Path::new(&output_as_path_buf).to_path_buf()
        };
        new_config.paths.output = output_path;

        // Update the 'rust-modules' folder to the new output folder.
        Command::new("mv")
            .arg(&new_config.paths.godot.join("rust-modules"))
            .arg(&new_config.paths.godot.join(&new_config.paths.output))
            .output()
            .expect("Unable to move output to the new directory");

        // Change the path to the dynamic libraries in the gdnlib file.
        let targets_str: Vec<&str> = new_config
            .general
            .targets
            .iter()
            .map(AsRef::as_ref)
            .collect();
        let gdnlib = content::create_gdnlib_file(
            &new_config.general.name,
            &diff_paths(&new_config.paths.output, &new_config.paths.godot)
                .expect("Unable to get path diff"),
            &targets_str,
        );

        // Finally create the new gndlib file and write to it.
        let gdnlib_file_name = format!("{}.gdnlib", &new_config.general.name);

        let gdnlib_path = &new_config.paths.output;
        std::fs::File::create(&gdnlib_path.join(&gdnlib_file_name))
            .expect("Unable to create gdnlib file");

        match write(&new_config.paths.output.join(gdnlib_file_name), gdnlib) {
            Ok(_v) => (),
            Err(e) => {
                println!("There was a problem creating the gdnlib file: {}", e);
                exit(1);
            }
        }

        // We're done with the config for now so we can write it over the original.
        let new_config_string =
            toml::to_string(&new_config).expect("Unable to convert v2.x config to string");
        match write(&config_path, new_config_string) {
            Ok(_v) => (),
            Err(e) => {
                println!(
                    "{}: {}",
                    "There was a problem writing to the v2.x config file".red(),
                    e
                );
                exit(1);
            }
        }

        // At some point godot_rust_helper_extensions was changed to godot_rust_helper_ext so we gotta change any reference to it in the Cargo.toml file.
        let mut cargo_string = read_to_string("Cargo.toml").expect("Unable to read Cargo.toml");
        if cargo_string.contains("godot_rust_helper_extensions") {
            cargo_string =
                cargo_string.replace("godot_rust_helper_extensions", "godot_rust_helper_ext");

            match write("Cargo.toml", cargo_string) {
                Ok(_v) => (),
                Err(e) => {
                    println!(
                        "{}: {}",
                        "There was a problem writing to the v2.x Cargo.toml file".red(),
                        e
                    );
                    exit(1);
                }
            }
        }

        // Now we gotta go through each of the components created and check to see if they are still using `use godot_rust_helper_extensions` and change it to ext just like above.
        for module in new_config.general.modules {
            let module_filename = format!("{}.rs", module.to_case(Case::Snake));
            let mut module_path = PathBuf::from("src");
            module_path.push(module_filename);

            let mut module_string = read_to_string(&module_path).expect("Unable to read module");
            // If the module has the old extensions, then update it to the new ones.
            if module_string.contains("godot_rust_helper_extensions") {
                module_string =
                    module_string.replace("godot_rust_helper_extensions", "godot_rust_helper_ext");

                match write(&module_path, module_string) {
                    Ok(_v) => (),
                    Err(e) => {
                        println!(
                            "{}: {}",
                            "there was a problem writing to the module file".red(),
                            e
                        );
                        exit(1);
                    }
                }
            }
        }
    }
    if !config_string.contains("nativescript") {
        config_string = read_to_string(&config_path)
            .expect("Unable to read godot-rust-helper.toml config file");
        // Parse the current configuration of the project as configuration v2.
        let current_config: ConfigV2 = toml::from_str(&config_string).expect(
            "Unable to parse godot-rust-helper config file of project using godot_rust_helper v2.x",
        );

        // Create an instance of the new configuration of the project using the values from the current config.
        let new_config_paths = ConfigPaths {
            lib: current_config.paths.lib,
            godot: current_config.paths.godot.to_owned(),
            output: utils::absolute_path(current_config.paths.output).expect(
                "Unable to create absolute path from from godot_rust_helper v2.x output path",
            ),
            nativescript: current_config.paths.godot.to_owned(),
        };
        let new_config_general = ConfigGeneralV3 {
            name: current_config.general.name,
            targets: current_config.general.targets,
            modules: current_config.general.modules,
        };
        let mut new_config = ConfigV3 {
            general: new_config_general,
            paths: new_config_paths,
        };

        // Check to see if the user would like to place the nativescript files in a specific directory.
        // If no path is provided for the nativescript directory then the files will be placed in the root of the project.
        let ns_path_as_path_buf = PathBuf::from(nativescript_path);
        let ns_path = if ns_path_as_path_buf == PathBuf::from("") {
            PathBuf::from(&new_config.paths.godot)
        } else if !ns_path_as_path_buf.is_absolute() {
            utils::absolute_path(ns_path_as_path_buf)
                .expect("Unable to create absolute path from nativescript path")
                .as_path()
                .to_owned()
        } else {
            Path::new(&ns_path_as_path_buf).to_path_buf()
        };
        new_config.paths.nativescript = ns_path;

        // We're done making changes to the config so we can write over the original now.
        // We're done with the config for now so we can write it over the original.
        let new_config_string =
            toml::to_string(&new_config).expect("Unable to convert v3.x config to string");
        match write(&config_path, new_config_string) {
            Ok(_v) => (),
            Err(e) => {
                println!(
                    "{}: {}",
                    "There was a problem writing to the v3.x config file".red(),
                    e
                );
                exit(1);
            }
        }
    }
    if !config_string.contains("plugin") {
        config_string = read_to_string(&config_path)
            .expect("Unable to read godot-rust-helper.toml config file");
        // Parse the current configuration of the project as configuration v3.
        let current_config: ConfigV3 = toml::from_str(&config_string).expect(
            "Unable to parse godot-rust-helper config file of project using godot_rust_helper v3.x",
        );

        // Create an instance of the new configuration of the project using the values from the current config.
        let new_config_paths = ConfigPaths {
            lib: current_config.paths.lib,
            godot: current_config.paths.godot.to_owned(),
            output: utils::absolute_path(current_config.paths.output).expect(
                "Unable to create absolute path from from godot_rust_helper v3.x output path",
            ),
            nativescript: current_config.paths.godot.to_owned(),
        };
        let new_config_general = ConfigGeneral {
            name: current_config.general.name,
            targets: current_config.general.targets,
            modules: current_config.general.modules,
            plugin: false,
        };
        let new_config = Config {
            general: new_config_general,
            paths: new_config_paths,
        };

        // We're done making changes to the config so we can write over the original now.
        // We're done with the config for now so we can write it over the original.
        let new_config_string =
            toml::to_string(&new_config).expect("Unable to convert v4.x config to string");
        match write(config_path, new_config_string) {
            Ok(_v) => (),
            Err(e) => {
                println!(
                    "{}: {}",
                    "There was a problem writing to the v4.x config file".red(),
                    e
                );
                exit(1);
            }
        }
    }

    println!("{}", "Update finished".green());
}

/// Creates a plugin similar to using `godot_rust_helper new` by creating the base file structure and the plugin.cfg file that allows us to create
/// a Nativescript plugin.
///
/// # Arguments
///
/// `name` - The name of the plugin.
/// `destination` - The destination directory for the library.
/// `godot_project_dir` - The directory that contains the Godot project that the plugin is for.
/// `description` - A short description of the plugin.
/// `author` - The author of the plugin. If no author is provided then the author field from the Cargo.toml is used.
/// `version` - The initial version of the plugin.
/// `targets` - The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
pub fn create_plugin(
    name: String,
    destination: PathBuf,
    godot_project_dir: PathBuf,
    description: String,
    author: String,
    version: String,
    targets: String,
) {
    println!("{}", "creating plugin".white());

    // Make the destination directory is an absolute path if it is not already one.
    let dest_path = if !destination.is_absolute() {
        utils::absolute_path(destination)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&destination).to_path_buf()
    };

    // Make the godot directory is an absolute path if it is not already one.
    let godot_path = if !godot_project_dir.is_absolute() {
        utils::absolute_path(godot_project_dir)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&godot_project_dir).to_path_buf()
    };

    // Check to see if the destination directory already exists, we don't want to overwrite an existing project.
    if dest_path.exists() {
        println!("A library with the specified destination already exists, please choose another destination for the library.");
        exit(1);
    }

    // Check to make sure that the provided path to the Godot project is valid, i.e. it has a project.godot file at its root.
    let godot_project_path = godot_path.join("project.godot");
    if !godot_project_path.exists() {
        println!("The godot project dir provided is not valid.");
        exit(1);
    }

    // Run the `cargo new --lib` command in the destination directory to create the library.
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
            println!("{}", e);
            exit(1);
        }
    }

    // Set up the Cargo.toml file of the library to have the required tags and dependencies.
    set_current_dir(&dest_basename).expect("Unable to change to library directory");
    let cargo_toml_string = read_to_string("Cargo.toml").expect("Unable to read Cargo.toml");
    let cargo_toml: Cargo = toml::from_str(&cargo_toml_string).expect("Unable to parse Cargo.toml");
    let cargo_toml_str = toml::to_string(&cargo_toml).expect("Unable to convert to toml to string");

    match write(
        "Cargo.toml",
        cargo_toml_str
            .replace("\\", "")
            .replace("\"{", "{")
            .replace("}\"", "}"),
    ) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the library: {}", e);
            exit(1);
        }
    }

    // Make sure the targets provided are in the list of accepted targets.
    let valid_targets = &["windows", "linux", "osx"];
    let targets_split: Vec<String> = targets.split(",").map(|s| s.to_string()).collect();
    for t in &targets_split {
        if !valid_targets.iter().any(|&i| i == t) {
            println!("An invalid target was specified: {}", t);
            exit(1);
        }
    }

    // Create all of the variations of the plugin names we'll need.
    let plugin_name = &name;
    let plugin_name_normalized = plugin_name.to_case(Case::Snake);
    // Create all of the paths we'll need for the plugin files.
    let plugin_path = godot_path.join("addons").join(&plugin_name_normalized);
    let plugin_cfg_path = plugin_path.join("plugin.cfg");

    // Create the folder structure for the plugin if it doesn't already exist.
    match std::fs::create_dir_all(&plugin_path) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "There was a problem creating the the plugin directory structure: {}",
                e
            );
            exit(1);
        }
    }

    // Create the config and write it to the godot-rust-helper.toml file.
    let config_paths = ConfigPaths {
        lib: dest_path.to_owned(),
        godot: godot_path.to_owned(),
        output: plugin_path.to_owned(),
        nativescript: plugin_path.to_owned(),
    };
    let config_general = ConfigGeneral {
        name: dest_basename_string.to_string(),
        modules: vec![],
        targets: targets_split.to_owned(),
        plugin: true,
    };
    let config = Config {
        general: config_general,
        paths: config_paths,
    };
    let config_string = toml::to_string(&config).expect("Unable to convert config to string");
    match write("godot-rust-helper.toml", config_string) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the config file: {}", e);
            exit(1);
        }
    }

    // Create the gdnlib file and write it to the plugin folder.
    let targets_str: Vec<&str> = targets_split.iter().map(AsRef::as_ref).collect();
    let gdnlib = content::create_gdnlib_file(
        &config.general.name,
        &diff_paths(&config.paths.output, &config.paths.godot)
            .expect("Unable to get path diff for the plugin"),
        &targets_str,
    );

    // Finally create the new gndlib file and write to it.
    let gdnlib_file_name = format!("{}.gdnlib", &config.general.name);

    let gdnlib_path = &config.paths.output;
    std::fs::File::create(&gdnlib_path.join(&gdnlib_file_name))
        .expect("Unable to create gdnlib file for the plugin");

    match write(&config.paths.output.join(gdnlib_file_name), gdnlib) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "There was a problem creating the gdnlib file for the plugin: {}",
                e
            );
            exit(1);
        }
    }

    // Create the plugin.cfg file and write it to the plugin folder.
    let plugin_cfg_fields = PluginConfigFields {
        name: name.to_owned(),
        description: description,
        author: author,
        version: version,
        script: format!("{}.gdns", plugin_name_normalized),
    };
    let plugin_cfg = PluginConfig {
        plugin: plugin_cfg_fields,
    };
    let plugin_cfg_string =
        toml::to_string(&plugin_cfg).expect("Unable to convert plugin config to string");
    match write(plugin_cfg_path, plugin_cfg_string) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the plugin config file: {}", e);
            exit(1);
        }
    }

    // Run `cargo create` to create the module's base script file that the configuration expects.
    match Command::new("godot_rust_helper")
        .arg("create")
        .arg(plugin_name.to_case(Case::Pascal))
        .output()
    {
        Ok(_v) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
    // Since this base script is a bit different, all instances of Node need to be swapped with EditorPlugin and then we write it back.
    let base_plugin_script_path = format!("src/{}.rs", name.to_case(Case::Snake));
    let base_plugin_script =
        read_to_string(&base_plugin_script_path).expect("Unable to read plugin's base script");
    let updated_base_plugin_script = base_plugin_script.replace("Node", "EditorPlugin");
    match write(base_plugin_script_path, updated_base_plugin_script) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "There was a problem modifying the plugin's base script: {}",
                e
            );
            exit(1);
        }
    }
    println!("{}", "plugin created".white());
}

/// Runs the build command and logs some info used by `watch_library` to show the version of godot_rust_helper and the timestamp of when the last build was run.
fn build_with_time_log() {
    let dt: DateTime<Local> = Local::now();
    let dt_formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    build_library();

    println!("");
    println!(
        "[{}] {}",
        dt_formatted.white(),
        "waiting for changes...".white()
    );
}
