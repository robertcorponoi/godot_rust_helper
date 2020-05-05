use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// The structure of the Cargo.toml file created by the `godot-rust-helper new` command.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cargo {
	/// A reference to the CargoPackage struct.
	package: CargoPackage,
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
	name: String,
	/// The version of the package.
	version: String,
	/// The authors of the package.
	authors: Vec<String>,
	/// The Rust edition used in the package.
	edition: String,
}

/// The fields of the Cargo.toml that are under the [lib] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoLib {
	/// The type of crate it is.
	#[serde(rename = "crate-type")]
	crate_type: Vec<String>,
}

/// The fields of the Cargo.toml that are under the [dependencies] tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoDependencies {
	/// The gdnative dependency that is required to create Rust modules.
	#[serde(default = "add_gdnative_dep")]
	gdnative: String,
	/// Optional helper methods.
	#[serde(default = "add_extensions_dep")]
	pub godot_rust_helper_extensions: String,
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
	return String::from("{ git = \"https://github.com/GodotNativeTools/godot-rust\" }");
}

/// Returns the godot_rust_helper_extensions dependency to add to the Cargo.toml dependencies.
pub fn add_extensions_dep() -> String {
	return String::from("{ git = \"https://github.com/robertcorponoi/godot-rust-helper-extensions\" }");
}

/// The structure of the godot-rust-helper.toml config file created by the `godot-rust-helper new`
/// command and used throughout the rest of the commands.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub general: ConfigGeneral,
}

// For now, everything will go under the [general] tag and the structure of it is defined below.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGeneral {
	/// The name of the library.
	pub name: String,
	/// The path to the library.
	pub lib_path: PathBuf,
	// The path to the godot project.
	pub godot_path: PathBuf,
	/// The build targets.
	pub targets: Vec<String>,
	/// The modules that have been created.
	pub modules: Vec<String>,
}
