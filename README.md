<p align="center">
  <img width="250" height="250" src="https://raw.githubusercontent.com/robertcorponoi/graphics/master/godot-rust-helper/godot-rust-helper-logo.png">
</p>

<h1 align="center">Godot Rust Helper</h1>

<p align="center">A simple CLI tool to help you create and update Rust components for your Godot projects.<p>

## **Install**

```bash
$ cargo install godot_rust_helper
```

To upgrade:

```bash
$ cargo install --force godot_rust_helper
```

Note: This documentation is for version 4.x. Documentation for previous versions can be found below:

- [3.x](https://github.com/robertcorponoi/godot_rust_helper/tree/v3.0.0)
- [2.x](https://github.com/robertcorponoi/godot_rust_helper/tree/v2.1.0)
- [1.x](https://github.com/robertcorponoi/godot_rust_helper/tree/v1.1.0)

## **Table of Contents**

- [Full Example](#full-example)
- [Commands](#commands)
  - [new](#new)
  - [create](#create)
  - [destroy](#destroy)
  - [build](#build)
  - [plugin](#plugin)
  - [update](#update)
  - [rebase](#rebase)

## **Full Example**

Below is a basic guide to setting up Rust with a Godot project from start to finish.

## **Step 1: Creating the Project's Library**

For each game you create in Godot you will have to create a new library. The library itself is a cargo library and it holds all of the components used in your game.

To create the project's library, navigate to where you would like to store the components (outside of your Godot project directory) and use the `new` command:

```bash
$ godot_rust_helper new <destination> <path_to_godot_project> [options]
```

Let's go over the arguments and options in detail with some examples.

**Arguments:**

- **library_name** The name of the library that will contain your Rust components. The name of the library is recommended to be the same or similar in name to your game. Also keep in mind that the library is created using `cargo new` so you should abide by the cargo project naming standards.
- **path_to_godot_project** This is the path to the root directory of the Godot project that the components will belong to.

**Options:**
- `--targets` Native components in Godot can target multiple platforms and godot_rust_helper needs to know ahead of time what platforms you plan to target your components for with the available options currently being: windows, linux, and osx. For example if you are targeting Windows and OSX, you need to have have cargo set to build a dll and a dylib file and you would pass `--targets=windows,osx` as the targets. By default if no targets are passed then just `--targets=windows` will be set.
- `--output-path` godot_rust_helper has to place a gdnlib file and the build files in the game's directory. By default these files are placed at the root of the game directory but you can specify a directory in the game (existing or not) where these files go instead using this option.
- `--nativescript-path` The path in the Godot project where all of the nativescript files will be output. By default the nativescript files are placed at the root of the Godot project.

**examples:**

Creating a default library for Windows only builds:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout
```

Creating an library for Windows, Linux, and OSX builds:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout --targets=windows,linux,osx
```

Creating a library and having the files output to `build-output`:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout --output-path ~/Documents/projects/breakout/build-output
```

Creating a library and having the nativescript files output to `scripts`:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout --nativescript-path ~/Documents/projects/breakout/scripts
```

**Note:** The `src/lib.rs` file is completely managed by godot_rust_helper and should not be modified. Any modifications to the file will result in the components not functioning properly or they will be overwritten when a module is created/destroyed. Custom mods can be added to the file (coming soon).

**Note:** Each instance of the library comes with `godot_rust_helper_extensions` as a dependency which is going to contain methods to make things easier (such as getting typed nodes) and include methods that are not a part of gdnative but are in gdscript. You do not have to use any extensions if you don't want to but if you are interested in them, check out the extensions [here](https://github.com/robertcorponoi/godot_rust_helper_ext).

If you are getting an error that says that the crate doesn't exist it's because the name of the extension was not right the first time around and it was fixed 1.0.2. To fix it you can update and create the library again or simply go to your Cargo.toml and change `godot_rust_helper_extensions` to `godot_rust_helper_ext`.

## **Step 2: Creating Components**

Now that you've created the library, you can go into the newly created folder and see the config file. This config file contains the path to the Godot project directory and the targets passed from the `new` command. This config file should not be modified manually as godot_rust_helper depends on it heavily.

From this directory, we can now begin to make components with the create command like so:

```bash
$ godot_rust_helper create <class_name>
```

- **name** The name passed to this command should be the class name of the component. Class names must start with capital letters. Examples include 'Player', 'Princess', 'Mob', 'HUD', etc.

What this does is create a `src/<name>.rs` file for the component and adds an entry for it in the `src/lib.rs` file. If you attach this component as it is to a Node and run the game then "hello, world" will print to the godot console. This also creates a `<name>.gdns` file at the location specified by `--nativescript-path` when you created the library. This is the script you attach to your Node in Godot.

**Note:** This command has to be run from the library's directory.

**examples:**

```bash
$ godot_rust_helper create Player
```

```bash
$ godot_rust_helper create MainScene
```

## **Step 3: Building the Library**

After you have created your component (or you can do this with the default contents to try it out) you're ready to run a build using:

```bash
$ godot_rust_helper build
```

What this does is first run `cargo build` and then it moves the build files into the Godot project directory.

**Note:** This command has to be run from the library's directory.

**Note:** The first time you run this it will take a while as it have to reach out and download the necessary dependencies, every build after that will be much quicker.

The build command also supports the `--watch` option which will watch the src directory of your component for changes and re-build it automatically.

**examples:**

Running the build command:

```bash
$ godot_rust_helper build
```

Running the build command and watching for changes to any components in the library.

```bash
$ godot_rust_helper build --watch
```

## **Step 4: Using the Components in Godot**

The last step is to attach the scripts to the Nodes in Godot:

1. Choose the node to add the component to and in the inspector go to the script dropdown and choose load.
2. Find the script to load in the root directory or the directory specified by `--nativescript-path` when the library was created and select it.

Now if you run your game you will see your component's functionality up and running!

**Note:** If you update your Rust component and run a build you do not have to update the corresponding .gdnlib file in Godot, it will be updated automatically.

**Note:** You do not need to keep your .gdns scripts in any certain place so feel free to move them around. As long as the gdnlib and dynamic library files are not moved then the nativescript files can be placed anywhere in the Godot project.

## **Commands**

### **new**

Creates a new library for your Rust scripts that connects to a Godot project.

```
Usage: godot_rust_helper new <destination> <godot-project> [options]

destination:                    The destination directory for the library. Note that libraries are created using cargo so you should adhere to cargo naming guidelines and use underscores for multiple words.
godot-project:                  The directory of the Godot project that this library contains the Rust scripts for.

Options:
-t, --targets <targets>         A string of comma separated targets of the platforms you would like to build the project for. Currently the available options are windows, linux, and osx with a default value of just windows.
-o, --output-path <path>        The path within the Godot project where the gdnlib and dynamic libraries will get output to. By default these files will be output to the root of the Godot project.
-n, --nativescript-path <path>  The path within the Godot project where the gdns files will be output to. By default these files will be output to the root of the Godot project.
```

**examples:**

Creating a default library for Windows only builds:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout
```

Creating an library for Windows, Linux, and OSX builds:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout --targets=windows,linux,osx
```

Creating a library and having the files output to `build-output`:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout --output-path ~/Documents/projects/breakout/build-output
```

Creating a library and having the nativescript files output to `scripts`:

```bash
$ godot_rust_helper new breakout_components ~/Documents/projects/breakout --nativescript-path ~/Documents/projects/breakout/scripts
```

## **create**

Creates a Rust script and a corresponding gdns file in the Godot project that when build can be placed on a Node.

```
Usage: godot_rust_helper create <class-name>

class-name  The name passed to this command should be the class name of the component. Class names must start with capital letters. Examples include 'Player', 'Princess', 'Mob', 'HUD', etc.
```

**examples:**

```bash
$ godot_rust_helper create Player
```

```bash
$ godot_rust_helper create MainScene
```

## **destroy**

Removes all traces of a script created with `create`.

```
Usage: godot_rust_helper destroy <class-name>

class-name The name of the class to destroy. This should be the same name that was used when it was created with `godot_rust_helper create`.
```

**examples:**

```bash
$ godot_rust_helper destroy Player
```

```bash
$ godot_rust_helper destroy MainScene
```

## **build**

Builds the project to generate the dynamic libraries and then copies them to the Godot project `output-path` directory.

```
Usage: godot_rust_helper build [options]

Options:
-w,--watch  Watches the src directory of the library for changes and runs the build command automatically.
```

**examples:**

```bash
$ godot_rust_helper build
```

```bash
$ godot_rust_helper build --watch
```

## **plugin**

Creates a library intended to be used as a plugin. This creates the directory structure for the plugin (addons/plugin-name) and also creates the plugin configuration file and the base plugin script.

```
Usage: godot_rust_helper plugin <name> <destination> <godot-project> [options]

name                           The name of the plugin. If the plugin consists of more than 1 word then it needs to be in quotes.
destination                    The destination directory for the library. Note that libraries are created using cargo so you should adhere to cargo naming guidelines and use underscores for multiple words.
godot-project                  The directory of the Godot project that this library contains the Rust scripts for.

Options:
-t, --targets <targets>         A string of comma separated targets of the platforms you would like to build the project for. Currently the available options are windows, linux, and osx with a default value of just windows.
-a, --author <author>           The author of the plugin.
-d, --description <description> The description of the plugin.
-v, --version <version>         The initial version of the plugin. If no version is provided then "1.0" will be used.
```

**example:**

```bash
$ godot_rust_helper plugin "Directory Browser" directory_browser ../path/to/godot/game --description "Helps you map out your game's file structure" --author "Bob"
```

### **update**

Updates a project from using and older version of godot_rust_helper to using the latest version of godot_rust_helper.

This command has to be used from inside the project you want to update.

**Note:** In version 4.x, gdns files are now output as snake case instead of pascal case. The update commmand does update your existing gdns file names because it would case lots of issues within the Godot project so you can leave them as they are or update them manually. 

```
Usage: godot_rust_helper update [options]

Options:
output-path       Since version 2.x, godot_rust_rust doesn't create a rust-modules folder you can specify this to change the location where the gdnlib and dynamic libraries reside. If left blank, the rust-modules folder will be used by default.
nativescript-path Since version 3.x, godot_rust_helper lets you spcify the directory where your .gdns files get output to.
```

**examples:**

Leaving the rust-modules folder (if you're updating from 1.x or 2.x to latest):

```bash
$ godot_rust_helper update
```

Moving the output files to a new directory:

```bash
$ godot_rust_helper update --output-path /path/to/godot-project/gdr-output
```

Specifying a directory for the gdns files:

```bash
$ godot_rust_helper update --output-path /path/to/godot-project/gdr-output --nativescript-path /path/to/godot-project/gdr-scripts
```

**Note:** You will probably have to run another build and you will definitely have to reassign the scripts to the gdnlib file after updating.

### **rebase**

The rebase command is useful if you import someone else's godot_rust_helper project and want to modify it locally.

This command lets you change the location of the godot project and the targets.

This command has to be used from inside the project you want to rebase.

```
Usage: godot_rust_helper rebase <new-godot-path> [options]

new-godot-path The path to where the Godot project that this library is for is on your file system.

Options:
targets The new build targets for the project, this should be used like it is in the `new` command.
```

**examples:**

```bash
$ godot_rust_helper rebase ../path/to/game
```

```bash
$ godot_rust_helper rebase ../path/to/game --targets=linux,osx
```

## **Tests**

```bash
$ cargo test -- --test-threads=1
```

## **License**

MIT