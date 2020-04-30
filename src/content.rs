use crate::utils;

use std::borrow::Cow;

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
    let mod_file = format!(
        r#"#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
pub struct {};
#[gdnative::methods]
impl {} {{
    fn _init(_owner: gdnative::Node) -> Self {{
    {}
    }}
    #[export]
    fn _ready(&self, _owner: gdnative::Node) {{
    godot_print!("hello, world.")
    }}
}}
`"#,
        name, name, name
    );

    return mod_file;
}

/// Returns the contents of the gdnlib file for the library.
///
/// # Arguments
///
/// `name` - The name of the library.
/// `targets` - The build targets of the library.
pub fn create_gdnlib_file(name: &str, targets: &[&str]) -> String {
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

    for &t in targets {
        match t {
            "windows" => {
                let file_name = format!("Windows.64=\"res://rust-modules/{}.dll\"", name).into();
                let dep_entry = "Windows.64=[  ]".into();

                gdnlib_vec.insert(entry_insert_point, file_name);
                gdnlib_vec.insert(dep_insert_point, dep_entry);
            }
            "linux" => {
                let file_name = format!("X11.64=\"res://rust-modules/lib{}.so\"", name).into();
                let dep_entry = "X11.64=[  ]".into();

                gdnlib_vec.insert(entry_insert_point, file_name);
                gdnlib_vec.insert(dep_insert_point, dep_entry);
            }
            "osx" => {
                let file_name = format!("OSX.64=\"res://rust-modules/lib{}.dylib\"", name).into();
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
