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

Note: This documentation is for version 3.x. Documentation for versions 2.x can be found [here](https://github.com/robertcorponoi/godot_rust_helper/tree/v2.1.0) and versions 1.x can be found [here](https://github.com/robertcorponoi/godot_rust_helper/tree/v1.1.0).

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
-`--output-path` godot_rust_helper has to place a gdnlib file and the build files in the game's directory. By default these files are placed at the root of the game directory but you can specify a directory in the game (existing or not) where these files go instead using this option.
-`--nativescript-path` The path in the Godot project where all of the nativescript files will be output. By default the nativescript files are placed at the root of the Godot project.

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
$ godot_rust_helper create HUD
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

## **Other Commands**

The following are commands are situational but are not needed for the basic setup.

### **destroy**

Removes a Rust component from the library.

```bash
$ godot_rust_helper destroy <class_name>
```

- **class_name** The name of the class to destroy. This should be the same name that was used when it was created with `godot_rust_helper create`.

**examples:**

```bash
$ godot_rust_helper destroy Player
```

```bash
$ godot_rust_helper destroy HUD
```

### **rebase**

Changes the path of the project, godot project directory, and optionally the targets in the config if you cloned/downlaoded the project from elsewhere.

This command has to be used from inside the project you want to rebase.

```bash
$ godot_rust_helper rebase <path_to_game> [targets]
```

- **path_to_game** The path to godot game on your file system.
- **targets** Optionally change the targets.

**examples:**

```bash
$ godot_rust_helper rebase ../path/to/game
```

```bash
$ godot_rust_helper rebase ../path/to/game --targets=linux,osx
```

### **update**

Updates a project from using godot_rust_helper 1.x or 2.x to godot_rust_helper 3.x.

This command has to be used from inside the project you want to update.

```bash
$ godot_rust_helper update [output-path]
```

- **output-path** Since godot_rust_helper 2.x doesn't create a rust-modules folder you can specify this to change the location where the gdnlib and build files reside. If left blank, the rust-modules folder will be used by default.
- **nativescript-path** Since godot_rust_helper 3.x you can specify the directory where your nativescript files are output.

**examples:**

Leaving the rust-modules folder:

```bash
$ godot_rust_helper update
```

Moving the output files to a new directory:

```bash
$ godot_rust_helper update --output-path /path/to/godot-project/gdr-output
```

**Note:** You will probably have to run another build and you will definitely have to reassign the scripts to the gdnlib file after updating.

## **Tests**

```bash
$ cargo test -- --test-threads=1
```

## **License**

MIT