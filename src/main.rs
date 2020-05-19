#[macro_use]

mod commands;
mod configs;
mod content;
mod utils;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
	about = "A simple CLI tool to help you create and update Rust modules for your Godot projects"
)]
enum GodotRustHelper {
	/// Creates the library that will contain your Rust modules.
	/// The name of the library that will contain your Rust modules. The name of the library is recommended to be the same or similar in name to your game.
	/// Also keep in mind that the library is created using `cargo new` so you should abide by the cargo project naming standards.
	New {
		/// The name of the library that will contain your Rust modules. The name of the library is recommended to be the same name as your game, snake_case,
		/// maybe with `_modules` at the end. Also keep in mind that the library is created using `cargo new`
		#[structopt(parse(from_os_str))]
		destination: PathBuf,
		/// The directory that contains the project.godot file of the game that the modules are for.
		#[structopt(parse(from_os_str))]
		godot_project_dir: PathBuf,
		/// The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
		#[structopt(long, short, default_value = "windows")]
		targets: String,
		// godot_rust_helper needs to output certain files to the Godot project directory such as a gdnlib and the compiled files.
		// If you specify a directory for the output then the output files will go there, otherwise they will go in the root of the Godot project directory.
		#[structopt(long, short, default_value = "")]
		output_path: PathBuf,
	},
	/// Creates a new module inside of the library.
	/// The name passed to this command should be the class name of the module. Class names must start with capital letters. Examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
	Create {
		/// The class name of the module to create; examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
		#[structopt()]
		name: String,
	},
	/// Removes a module created with `create`.
	/// The name passed to this command should be the same name that was passed when the module was created.
	Destroy {
		/// The name of the module to destory.
		#[structopt()]
		name: String,
	},
	/// Runs the `cargo build` command and copies the build files to the Godot project.
	Build {
		// Indicates whether the godot_rust_helper should watch the project for changes and rebuild automatically or not.
		#[structopt(long, short)]
		watch: bool,
	},
	/// Changes the project path and the godot project path in the config and optionally sets new targets.
	/// This is useful if you cloned a project using godot_rust_helper.
	Rebase {
		/// The directory that contains the project.godot file of the game that the modules are for.
		#[structopt(parse(from_os_str))]
		godot_project_dir: std::path::PathBuf,
		/// The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
		#[structopt(long, short, default_value = "")]
		targets: String,
	},
	/// Update a library from using godot_rust_helper v1.x to v2.x.
	Update {
		/// As of godot_rust_helper 2.x the 'rust-modules' directory no longer exists and is customizable. You can change this to a different directory at this time but you'll have to fix all references in Godot.
		#[structopt(long, short, default_value = "")]
		output_path: PathBuf,
	},
}

fn main() {
	match GodotRustHelper::from_args() {
		// When the `new` command is used we run the `commands::create_library` function to create a new library for the Rust modules.
		GodotRustHelper::New {
			destination,
			godot_project_dir,
			targets,
			output_path,
		} => {
			commands::create_library(destination, godot_project_dir, targets, output_path);
		}
		// When the `create` command is used we run the `commands::create_module` function to create a module inside of the library.
		GodotRustHelper::Create { name } => {
			commands::create_module(&name.to_owned());
		}
		// When the `destroy` command is used we run the `commands::destory_module` function to remove a module inside of the library
		GodotRustHelper::Destroy { name } => {
			commands::destroy_module(&name.to_owned());
		}
		// When the `build` command is used we run the `commands::build_library` function to generate the build files and copy them to Godot project.
		GodotRustHelper::Build { watch } => {
			if watch {
				commands::watch_library()
			} else {
				commands::build_library()
			}
		}
		// When the `rebase` command is used we run the `commands::rebase` function to update the config file.
		GodotRustHelper::Rebase {
			godot_project_dir,
			targets,
		} => {
			commands::rebase(godot_project_dir, targets);
		}
		// When the `update` command is used we run the `commands::update` function to update the project from using godot_rust_helper 1.x to godot_rust_helper 2.x.
		GodotRustHelper::Update { output_path } => {
			commands::update(output_path);
		}
	}
}
