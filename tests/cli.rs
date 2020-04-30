use assert_cmd::prelude::*;
use predicates::prelude::*;

use std::process::Command;
use std::fs::read_to_string;

/// It should fail creating the library because the path provided to the Godot project is not valid.
#[test]
fn new_fail_no_project_godot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
        .arg("kinematic_character")
        .arg("non-existent-godot-game");

    cmd.assert().failure().stderr(predicate::str::contains(
        "The godot project dir provided is not valid.",
    ));

    Ok(())
}

// It should create a new environment with the default Cargo.toml file.
#[test]
fn new_has_correct_cargo_toml() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_current_dir("tests").expect("Unable to change to library directory");

    let mut cmd = Command::cargo_bin("godot_rust_helper")?;
    cmd.arg("new")
    .arg("platformer_modules")
    .arg("platformer");

    cmd.assert().success();

    let cargo_toml = read_to_string("platformer_modules/Cargo.toml").expect("Unable to read Cargo.toml");
    let cargo_toml_split = cargo_toml.split("\n");

    println!("{:?}", cargo_toml_split);
    //assert_eq!(cargo_toml, vec![]);

    Ok(())
}