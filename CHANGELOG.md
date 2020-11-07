## 5.0.0 / 2020-11-07
- [FEATURE] Updated gdnative dependency to its latest version.
- [FEATURE] Updated generated content to match the new API for the latest version of gdnative.
- [MISC] Updated tests.

## 4.0.2 / 2020-11-05
- [MISC] Updated dependencies to their latest versions.

## 4.0.1 / 2020-06-17
- [PATCH] Changed the gdnative dependency to use crates.io instead of the GitHub repo.

## 4.0.0 / 2020-05-30
- [FEATURE] Plugins can now be created with `godot_rust_helper plugin`. This prompted a major version change as the config file has a new entry that indicates whether a library is a plugin or not.
- [FEATURE] To better match most scripts in Godot, the gdns files are now output as snake_case.
- [README] Updated documentation to be easier to navigate.
- [MISC] Updated tests.

## 3.0.0 / 2020-05-22
- [FEATURE] Nativescript files now get created automatically when you use `godot_rust_helper create`.
- [FEATURE] Nativescript files output directory can be specified using the `--nativescript-path` option when creating a new library.

## 2.1.0 / 2020-05-19
- [FEATURE] Added update command to update projects from using godot_rust_helper 1.x to godot_rust_helper 2.x

## 2.0.0 / 2020-05-16
- [FEATURE] Output (gdnlib, build files) path can be specified using --output-path. This means that the rust-modules folder doesn't exist anymore and these files will be placed in the root directory by default.

## 1.1.0 / 2020-05-10
- [FEATURE] Added rebase command that allows you to easily update the config of another cloned/downloaded godot_rust_helper project.
- [MISC] Changed to using dunce for absolute path resolution.

## 1.0.2 / 2020-05-08
- [HOTFIX] Renamed godot_rust_helper_extensions to godot_rust_helper_ext.
- [DOCS] Added information about extensions.

## 1.0.1 / 2020-05-07
- [HOTFIX] Fixed an issue where you could build with errors and it would tell you that it built successfully.

## 1.0.0 / 2020-05-05
- Initial release