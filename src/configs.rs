use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The structure of the Cargo.toml file created by the `godot-rust-helper new` command.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cargo {
	/// A reference to the CargoPackage struct.
	pub package: CargoPackage,
	/// A reference to the CargoLib struct.
	/// If it doesn't exist (which it most likely won't) then we use the `create_cargo_lib` function to set the default value.
	#[serde(default = "create_cargo_lib")]
	pub lib: CargoLib,
	/// A reference to the CargoDependencies struct.
	pub dependencies: CargoDependencies,
}

/// The fields of the Cargo.toml that are under the [package] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoPackage {
	/// The name of the package.
	pub name: String,
	/// The version of the package.
	pub version: String,
	/// The authors of the package.
	pub authors: Vec<String>,
	/// The Rust edition used in the package.
	pub edition: String,
}

/// The fields of the Cargo.toml that are under the [lib] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoLib {
	/// The type of crate it is.
	#[serde(rename = "crate-type")]
	pub crate_type: Vec<String>,
}

/// The fields of the Cargo.toml that are under the [dependencies] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoDependencies {
	/// The gdnative dependency that is required to create Rust modules.
	#[serde(default = "add_gdnative_dep")]
	pub gdnative: String,
	/// Optional helper methods.
	#[serde(default = "add_extensions_dep")]
	pub godot_rust_helper_ext: String,
}

/// Returns the contents of what should appear under the [lib] tag.
/// This is used by the Cargo struct to create the default value for the [lib] tag if no value is present.
fn create_cargo_lib() -> CargoLib {
	return CargoLib {
		crate_type: vec!["cdylib".to_string()],
	};
}

/// Returns the gdnative dependency to add to the Cargo.toml dependencies.
/// This is used by the CargoDependencies struct to add the gdnative dependency that is necessary.
fn add_gdnative_dep() -> String {
	return String::from("0.9.1");
}

/// Returns the godot_rust_helper_extensions dependency to add to the Cargo.toml dependencies.
pub fn add_extensions_dep() -> String {
	return String::from("{ git = \"https://github.com/robertcorponoi/godot_rust_helper_ext\" }");
}

/// The structure of the plugin.cfg file used by `godot_rust_helper plugin`.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
	/// All of the plugins fields are under the [plugin] tag.
	pub plugin: PluginConfigFields,
}

/// Everything in the plugin config goes under a [plugin] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfigFields {
	/// The name of the plugin.
	pub name: String,
	/// The description of the plugin.
	pub description: String,
	/// The author of the plugin.
	pub author: String,
	/// The version of the plugin.
	pub version: String,
	/// The plugin's base script.
	pub script: String,
}

/// The structure of the godot-rust-helper.toml config file created by the `godot-rust-helper new`
/// command and used throughout the rest of the commands.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	/// General configuration options that can't be grouped in other ways (for now).
	pub general: ConfigGeneral,
	/// The locations of various important parts of the project.
	pub paths: ConfigPaths,
}

/// General configuration options that can't be grouped in other ways (for now).
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGeneral {
	/// The name of the library.
	pub name: String,
	/// The build targets.
	pub targets: Vec<String>,
	/// The modules that have been created.
	pub modules: Vec<String>,
	/// Indicates whether this is a plugin or not.
	pub plugin: bool,
}

/// Contains the location of the scripts, the godot project, and the folder in the godot project that contains the
/// gdnlib file and the build files.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigPaths {
	/// The path to the library that contains the Rust scripts.
	pub lib: PathBuf,
	/// The path to the Godot project.
	pub godot: PathBuf,
	/// The relative path to the gdnlib file and build files in Godot.
	pub output: PathBuf,
	/// The relative path to where the nativescript files in Godot.
	pub nativescript: PathBuf,
}

/// The v3.x version of the godot-rust-helper.toml config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigV3 {
	pub general: ConfigGeneralV3,
	pub paths: ConfigPaths,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGeneralV3 {
	/// The name of the library.
	pub name: String,
	/// The build targets.
	pub targets: Vec<String>,
	/// The modules that have been created.
	pub modules: Vec<String>,
}

/// The v2.x version of the godot-rust-helper.toml config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigV2 {
	pub general: ConfigGeneral,
	pub paths: ConfigPathsV2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigPathsV2 {
	pub lib: PathBuf,
	pub godot: PathBuf,
	pub output: PathBuf,
}

/// The v1.x version of the godot-rust-helper.toml config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigV1 {
	pub general: ConfigGeneralV1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGeneralV1 {
	pub name: String,
	pub lib_path: PathBuf,
	pub godot_path: PathBuf,
	pub targets: Vec<String>,
	pub modules: Vec<String>,
}
