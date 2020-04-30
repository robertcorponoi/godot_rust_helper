#[macro_use]
extern crate log;

mod utils;
mod commands;
mod configs;
mod content;

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
		destination: std::path::PathBuf,
		/// The directory that contains the project.godot file of the game that the modules are for.
		#[structopt(parse(from_os_str))]
		godot_project_dir: std::path::PathBuf,
		/// The build targets that should be set. As of writing this, the available targets are windows, linux, and osx with the default being just windows.
		#[structopt(long, short, default_value = "windows")]
		targets: String,
		// /// Indicates whether an optional extensions module will be added for ease of use functions such as getting typed nodes.
		// #[structopt(long, short)]
		// extensions: bool,
	},
	/// Creates a new module inside of the library.
	/// The name passed to this command should be the class name of the module. Class names must start with capital letters. Examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
	Create {
		/// The class name of the module to create; examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
		#[structopt()]
		name: String,
	},
}

fn main() {
	env_logger::init();
	match GodotRustHelper::from_args() {
		// When the `new` command is used we run the `commands::create_library` function to create a new library for the Rust modules.
		GodotRustHelper::New {
			destination,
			godot_project_dir,
			targets,
			// extensions,
		} => {
			commands::create_library(destination, godot_project_dir, targets);
		},
		// When the `create` command is used we run the `commands::create_module` function to create a module inside of the library.
		GodotRustHelper::Create {
			name,
		} => {
			commands::create_module(&name.to_owned());
		}
	}
}
