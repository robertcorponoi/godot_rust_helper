<p align="center">
  <img width="250" height="250" src="https://raw.githubusercontent.com/robertcorponoi/graphics/master/godot-rust-helper/godot-rust-helper-logo.png">
</p>

<h1 align="center">Godot Rust Helper</h1>

<p align="center">A simple CLI tool to help you create and update Rust modules for your Godot projects.<p>

## **Install**

```bash
$ cargo install godot_rust_helper
```

To upgrade:

```bash
$ cargo install --force godot_rust_helper
```

## **Step 1: Creating the Project's Library**

For each game you create in Godot you will have to create a new library. The library itself is a cargo library and it holds all of the modules used in your game.

To create the project's library, navigate to where you would like to store the modules (outside of your Godot project directory) and use the `new` command:

```bash
$ godot_rust_helper new <destination> <path_to_godot_project> [options]
```

Let's go over the arguments and options in detail with some examples.

**Arguments:**

- **library_name** The name of the library that will contain your Rust modules. The name of the library is recommended to be the same or similar in name to your game. Also keep in mind that the library is created using `cargo new` so you should abide by the cargo project naming standards.
- **path_to_godot_project** This is the path to the root directory of the Godot project that the modules will belong to.

**Options:**
- `--targets` Native modules in Godot can target multiple platforms and godot_rust_helper needs to know ahead of time what platforms you plan to target your modules for with the available options currently being: windows, linux, and osx. For example if you are targeting Windows and OSX, you need to have have cargo set to build a dll and a dylib file and you would pass `--targets=windows,osx` as the targets. By default if no targets are passed then just `--targets=windows` will be set.

**examples:**

Creating a default library for Windows only builds:

```bash
$ godot_rust_helper new breakout_modules ~/Documents/projects/breakout
```

Creating an library for Windows, Linux, and OSX builds:

```bash
$ godot_rust_helper new breakout-modules ~/Documents/projects/breakout windows,linux,osx
```

**Note:** The `src/lib.rs` file is completely managed by godot_rust_helper and should not be modified. Any modifications to the file will result in the modules not functioning properly or they will be overwritten when a module is created/destroyed. Custom mods can be added to the file (coming soon).

## **Step 2: Creating Modules**

Now that you've created the library, you can go into the newly created folder and see the config file. This config file contains the path to the Godot project directory and the targets passed from the `new` command. This config file should not be modified manually as godot_rust_helper depends on it heavily.

From this directory, we can now begin to make modules with the create command like so:

```bash
$ godot_rust_helper create <class_name>
```

- **name** The name passed to this command should be the class name of the module. Class names must start with capital letters. Examples include 'Player', 'Princess', 'Mob', 'HUD', etc.

What this does is create a `src/<name>.rs` file for the module and adds an entry for it in the `src/lib.rs` file. If you attach this script as it is to a Node and run the game then "hello, world" will print to the godot console.

**Note:** This command has to be run from the library's directory.

**examples:**

```bash
$ godot_rust_helper create Player
```

```bash
$ godot_rust_helper create HUD
```

## **Step 3: Building Modules**

After you have created your module (or you can do this with the default contents to try it out) you're ready to run a build using:

```bash
$ godot_rust_helper build
```

What this does is first run `cargo build` and then it moves the build files into the Godot project directory.

**Note:** This command has to be run from the library's directory.

**Note:** The first time you run this it will take a while as it have to reach out and download the necessary dependencies, every build after that will be much quicker.

The build command also supports the `--watch` option which will watch the src directory of your module for changes and re-build it automatically.

**examples:**

Running the build command:

```bash
$ godot_rust_helper build
```

Running the build command and watching for changes to any modules in the library.

```bash
$ godot_rust_helper build --watch
```

## **Step 4: Using the Module in Godot**

The last step that has to be done to use your module in your Godot project is creating the script and attaching it to the node that needs to use it.

After you have created a module and run a build, you can attach the script to a node like so:

1. Choose the node to add the script to and in the inspector go to the script dropdown and choose to add a new script.
2. In the Attach Node Script modal, set the following options:
  - **Language:** NativeScript
  - **Class Name:** The name you passed to `godot_rust_helper create` which is the class name of the Rust module you created.
3. Change the name of the script to match the class name and save the script to the rust-modules folder
4. Click on the newly created .gdns file (or after the steps above it should be active in the inspector already) and in the Library dropdown choose load and select the "library_name.gdnlib" file in the rust-modules folder. This library name is the same name passed to `godot_rust_helper new`.
4. Click on the newly created Node.gdns (or whatever you named it above if you chose a custom name).

Now if you run your game you will see your script's functionality up and running!

**Note:** If you update your Rust module you do not have to update the corresponding .gdnlib file in Godot, it will be updated automatically.

## **Other Commands**

The following are commands are situational but are not needed for the basic setup.

### **destroy**

Removes a Rust module from the library. You will still need to remove the script reference from your node in Godot as it will throw an error if you attempt to run the game since the script no longer exists.

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

## **Tests**

```bash
$ cargo test -- --test-threads=1
```

## **License**

MIT