use crate::utils;

use path_slash::PathBufExt;
use std::borrow::Cow;
use std::path::PathBuf;

/// Returns the initial contents of the src/lib.rs file.
pub fn create_initial_lib_file() -> String {
    return r#"#[macro_use]
extern crate gdnative;

fn init(handle: gdnative::init::InitHandle) {
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();"#
        .to_string();
}

/// Creates the contents for the src/lib.rs file depending on what modules are present.
///
/// # Arguments
///
/// `modules` - The modules that have been created.
pub fn create_lib_file(modules: &Vec<String>) -> String {
    let mut mods = String::new();
    let mut classes = String::new();

    for module in modules {
        let mod_normalized = utils::format_str(module.to_string());
        let mod_formatted = format!("\nmod {};", mod_normalized);
        let class_formatted = format!("\thandle.add_class::<{}::{}>();", mod_normalized, module);

        mods.push_str(&mod_formatted.to_owned());
        classes.push_str(&class_formatted.to_owned());

        let mod_index = modules
            .iter()
            .position(|i| i == module)
            .expect("Unable to get position of module");

        if mod_index != modules.iter().count() - 1 {
            classes.push_str("\n")
        }
    }
    mods.push_str("\n");

    let lib_file = format!(
        r#"#[macro_use]
extern crate gdnative;
{}
fn init(handle: gdnative::init::InitHandle) {{
{}
}}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();"#,
        mods, classes
    );

    return lib_file;
}

/// Creates the default contents of the module file for any module created.
///
/// # Arguments
///
/// `name` - The name of the module.
pub fn create_mod_file(name: &str) -> String {
    let init_string = format!(
        r#"fn _init(_owner: gdnative::Node) -> Self {{
{}
{}"#,
        format!("\t\t{}", name),
        "\t}"
    );

    let ready_string = format!(
        r#"#[export]
{}
{}
{}"#,
        "\tfn _ready(&self, _owner: gdnative::Node) {",
        format!("\t\t{}", "godot_print!(\"hello, world.\");"),
        "\t}"
    );

    let mod_file = format!(
        r#"#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
pub struct {};

#[gdnative::methods]
impl {} {{
{}

{}
}}
"#,
        name,
        name,
        format!("\t{}", init_string),
        format!("\t{}", ready_string)
    );

    return mod_file;
}

/// Returns the contents of the gdnlib file for the library.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `output_path` - The path where the compiled files are being output.
/// `targets` - The build targets of the library.
pub fn create_gdnlib_file(name: &str, output_path: &PathBuf, targets: &[&str]) -> String {
    let mut gdnlib_vec: Vec<Cow<str>> = vec![
        "[entry]".into(),
        "".into(),
        "".into(),
        "[dependencies]".into(),
        "".into(),
        "".into(),
        "[general]".into(),
        "".into(),
        "singleton=false".into(),
        "load_once=true".into(),
        "symbol_prefix=\"godot_\"".into(),
        "reloadable=true".into(),
        "".into(),
    ];

    let entry_insert_point = 2;
    let mut dep_insert_point = 6;

    let output_path_str = output_path
        .to_owned()
        .into_os_string()
        .into_string()
        .expect("Unable to create string from output_path");

    let mut o_path = PathBuf::from(output_path);

    for &t in targets {
        match t {
            "windows" => {
                let f_name = format!("{}.dll", name);
                o_path.push(f_name);

                let f_str = o_path.to_slash().unwrap();

                let file_name = format!("Windows.64=\"res://{}\"", f_str).into();
                let dep_entry = "Windows.64=[  ]".into();

                gdnlib_vec.insert(entry_insert_point, file_name);
                gdnlib_vec.insert(dep_insert_point, dep_entry);
            }
            "linux" => {
                let file_name =
                    format!("X11.64=\"res://{}lib{}.so\"", output_path_str, name).into();
                let dep_entry = "X11.64=[  ]".into();

                gdnlib_vec.insert(entry_insert_point, file_name);
                gdnlib_vec.insert(dep_insert_point, dep_entry);
            }
            "osx" => {
                let file_name =
                    format!("OSX.64=\"res://{}lib{}.dylib\"", output_path_str, name).into();
                let dep_entry = "OSX.64=[  ]".into();

                gdnlib_vec.insert(entry_insert_point, file_name);
                gdnlib_vec.insert(dep_insert_point, dep_entry);
            }
            _ => unimplemented!(),
        }
        dep_insert_point += 1;
    }

    return gdnlib_vec.join("\n");
}

/// Returns the contents of a class' .gdns file.
///
/// # Arugments
///
/// `lib_name` - The name of the library.
/// `class_name` - The name of the class.
/// `gdnlib_path` - The path to the gdnlib file.
pub fn create_gdns_file(lib_name: &str, class_name: &str, gdnlib_path: &PathBuf) -> String {
    let gdnlib_path_owned = gdnlib_path.to_owned();
    let gdnlib_os_str = gdnlib_path_owned.into_os_string();
    let gdnlib_path_str = gdnlib_os_str.to_str().expect("Unable to convert gdnlib path to str");

    let gdns_string = format!(
        r#"[gd_resource type="NativeScript" load_steps=2 format=2]

[ext_resource path="res://{}/{}.gdnlib" type="GDNativeLibrary" id=1]

[resource]

resource_name = "{}"
class_name = "{}"
library = ExtResource( 1 )
"#,
    gdnlib_path_str, lib_name, class_name, class_name,
    );

    return gdns_string;
}
