use std::env::{current_dir, set_current_dir};
use std::fs::{read_to_string, remove_file, write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::mpsc::channel;

use crate::configs::{Cargo, Config, ConfigGeneral, ConfigPaths, ConfigV1};
use crate::content;
use crate::utils;

use chrono::prelude::*;
use colored::*;
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use pathdiff::diff_paths;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Creates the library that will contain your Rust modules.
///
/// # Arguments
///
/// `destination` - The destination directory for the library.
/// `godot_project_dir` - The directory that contains the Godot project that the modules are for.
/// `targets` - The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
/// `output` - A directory within the godot project to place the gdnlib and build files.
pub fn create_library(
    destination: PathBuf,
    godot_project_dir: PathBuf,
    targets: String,
    output: PathBuf,
) {
    println!("{}", "creating library".white());

    // Make an absolute path out of the provided destination, godot, and output paths if they are not already.
    let dest_path = if !destination.is_absolute() {
        utils::absolute_path(destination)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&destination).to_path_buf()
    };
    let godot_path = if !godot_project_dir.is_absolute() {
        utils::absolute_path(godot_project_dir)
            .expect("Unable to create absolute path from destination path")
            .as_path()
            .to_owned()
    } else {
        Path::new(&godot_project_dir).to_path_buf()
    };
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

    // We don't need the full path to the output folder since it's relative to the godot path.
    let output_relative_path =
        diff_paths(&output_path, &godot_path).expect("Unable to get path diff");

    // Check to see if the library already exists.
    if dest_path.exists() {
        println!("A library with the specified destination already exists, please choose another destination for the library.");
        exit(1);
    }

    // Check to see if the path to Godot project is valid.
    // In order for it to be considered valid, it needs to have a project.godot file.
    let godot_project_path = godot_path.join("project.godot");
    if !godot_project_path.exists() {
        println!("The godot project dir provided is not valid.");
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
            println!("{}", e);
            exit(1);
        }
    }

    // Add the correct tags and dependencies to the library's Cargo.toml.
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

    // Check to see if the targets are valid.
    let valid_targets = &["windows", "linux", "osx"];
    let targets_split = targets.split(",").map(|s| s.to_string()).collect();

    for t in &targets_split {
        if !valid_targets.iter().any(|&i| i == t) {
            println!("An invalid target was specified: {}", t);
            exit(1);
        }
    }

    // Create the config and write it to a file.
    let config_paths = ConfigPaths {
        lib: dest_path.to_owned(),
        godot: godot_path.to_owned(),
        output: output_relative_path.to_owned(),
    };
    let config_general = ConfigGeneral {
        name: dest_basename_string.to_string(),
        modules: vec![],
        targets: targets_split,
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

    // Create the initial src/lib.rs file for the library.
    match write("src/lib.rs", content::create_initial_lib_file()) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem writing to the lib file: {}", e);
            exit(1);
        }
    }

    if !godot_path.is_absolute() {
        let current = current_dir().expect("Unable to get parent dir");
        let parent = current
            .as_path()
            .parent()
            .expect("Unable to get parent dir");
        let grandparent = parent.parent().expect("Unable to get grandparent dir");

        set_current_dir(&grandparent).expect("Unable to change to grandparent dir");
    }

    let output_full_path = &godot_path.join(output_path);

    match std::fs::create_dir_all(&output_full_path) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "There was a problem creating the the rust modules folder: {}",
                e
            );
            exit(1);
        }
    }

    let targets_vec: Vec<String> = targets.split(",").map(|s| s.to_string()).collect();
    let targets_str: Vec<&str> = targets_vec.iter().map(AsRef::as_ref).collect();

    let gdnlib =
        content::create_gdnlib_file(dest_basename_string, &output_relative_path, &targets_str);
    let gdnlib_file_name = format!("{}.gdnlib", dest_basename_string);

    std::fs::File::create(&output_full_path.join(&gdnlib_file_name))
        .expect("Unable to create gdnlib file");

    match write(output_full_path.join(gdnlib_file_name), gdnlib) {
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

    // Split the module name on capital letters and then make it all lowercase to use in some places.
    let name_normalized = utils::format_str(name.to_string());

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
    let lib_file = content::create_lib_file(&config.general.modules);
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
            println!("There was a problem creating the module: {}", e);
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

    // Split the module name on capital letters and then make it all lowercase to use when removing the module file.
    let name_normalized = utils::format_str(name.to_string());

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
        content::create_lib_file(&config.general.modules)
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
    for target in &config.general.targets {
        let ext = if target == "linux" {
            "so"
        } else if target == "osx" {
            "dylib"
        } else if target == "windows" {
            "dll"
        } else {
            exit(1);
        };
        let extra = if cfg!(windows) { "" } else { "lib" };

        let file = format!("{}{}.{}", extra, config.general.name, ext);
        let file_path = targets_dir.join(file);

        Command::new("cp")
            .arg(file_path)
            .arg(&config.paths.godot.join(&config.paths.output))
            .output()
            .expect("Unable to copy build files");

        println!("{}", "build complete".green());
    }
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

/// Updates a library from using godot_rust_helper 1.x to using godot_rust_helper 2.x.
///
/// # Arguments
///
/// `output` - As of godot_rust_helper 2.x the 'rust-modules' directory no longer exists and is customizable. You can change this to a different directory at this time but you'll have to fix all references in Godot.
pub fn update(output: PathBuf) {
    println!("{}", "Updating project from v1.x to v2.x...".white());

    // Check to see if we are in a library created by godot_rust_helper and at the same time we
    // get a reference to the config file so we can update it.
    let current_dir_path = current_dir().expect("Unable to get current directory");
    let config_path = current_dir_path.join("godot-rust-helper.toml");
    if !config_path.exists() {
        println!(
            "The upgrade command can only be used inside of a library created with the new command"
        );
        exit(1);
    }

    // Read and parse the old config.
    let config_string =
        read_to_string(&config_path).expect("Unable to read godot-rust-helper.toml config file");
    let old_config: ConfigV1 =
        toml::from_str(&config_string).expect("Unable to parse godot-rust-helper config");
    // Create the new config with values from the old config.
    let new_config_paths = ConfigPaths {
        lib: utils::absolute_path(old_config.general.lib_path)
            .expect("Unable to create absolute path from old lib path"),
        godot: utils::absolute_path(old_config.general.godot_path)
            .expect("Unable to create absolute path from old godot path"),
        output: PathBuf::new(),
    };
    let new_config_general = ConfigGeneral {
        name: old_config.general.name,
        targets: old_config.general.targets,
        modules: old_config.general.modules,
    };
    let mut new_config = Config {
        general: new_config_general,
        paths: new_config_paths,
    };
    // Check to see if the user would like to get rid of the
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

    let output_relative_path =
        diff_paths(&output_path, &new_config.paths.godot).expect("Unable to get path diff");
    new_config.paths.output = output_relative_path;

    // Update the rust-modules folder to the new output folder if necessary.
    Command::new("mv")
        .arg(&new_config.paths.godot.join("rust-modules"))
        .arg(&new_config.paths.godot.join(&new_config.paths.output))
        .output()
        .expect("Unable to move output folder");

    // Change the path value in the gdnlib file.
    let targets_str: Vec<&str> = new_config.general.targets.iter().map(AsRef::as_ref).collect();
    let gdnlib =
        content::create_gdnlib_file(&new_config.general.name, &new_config.paths.output, &targets_str);
    let gdnlib_file_name = format!("{}.gdnlib", &new_config.general.name);

    let gdnlib_path = &new_config.paths.godot.join(&new_config.paths.output);
    std::fs::File::create(&gdnlib_path.join(&gdnlib_file_name))
        .expect("Unable to create gdnlib file");

    match write(&new_config.paths.godot.join(&new_config.paths.output).join(gdnlib_file_name), gdnlib) {
        Ok(_v) => (),
        Err(e) => {
            println!("There was a problem creating the gdnlib file: {}", e);
            exit(1);
        }
    }

    // Finally write the new config to the same location as the old config.
    let new_config_string =
        toml::to_string(&new_config).expect("Unable to convert new config to string");
    match write(config_path, new_config_string) {
        Ok(_v) => (),
        Err(e) => {
            println!(
                "{}: {}",
                "There was a problem writing to the config file".red(),
                e
            );
            exit(1);
        }
    }

    // Next, we check to see if the Cargo.toml of the library contains a reference to the old extensions path.
    let mut cargo_string = read_to_string("Cargo.toml").expect("Unable to read Cargo.toml");
    if cargo_string.contains("godot_rust_helper_extensions") {
        cargo_string =
            cargo_string.replace("godot_rust_helper_extensions", "godot_rust_helper_ext");

        match write("Cargo.toml", cargo_string) {
            Ok(_v) => (),
            Err(e) => {
                println!(
                    "{}: {}",
                    "There was a problem writing to the Cargo.toml file".red(),
                    e
                );
                exit(1);
            }
        }
    }

    // Next, go through each of the modules created and see if there are any that are using the old extensions.
    for module in new_config.general.modules {
        let module_filename = format!("{}.rs", utils::format_str(module));
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

    println!("{}", "Update finished".green());
}

/// Runs the build command and logs the time.
/// This is used by the `watch_library` method to run the build and show the timestamp of when the build was last run.
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
