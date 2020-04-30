#[macro_use]
extern crate gdnative;

fn init(handle: gdnative::init::InitHandle) {
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();