extern crate path_clean;
extern crate regex;

use path_clean::PathClean;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
extern crate dunce;

/// Returns the path of the specified file.
///
/// # Arguments
///
/// `file_to_find` - The name of the file to find.
pub fn find_file(file_to_find: String) -> std::path::PathBuf {
    let mut exists = false;
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    let mut dir_to_check = std::path::Path::new(&current_dir);
    let mut iterations = 0;

    while !exists && iterations <= 10 {
        let temp_path = std::path::Path::new(&dir_to_check).join(&file_to_find);
        exists = temp_path.exists();

        if !exists {
            iterations += 1;
            dir_to_check = dir_to_check
                .parent()
                .expect("Unable to get parent directory");
        }
    }

    return dir_to_check.to_owned();
}

/// Splits a string on capitals, joins it back together with an underscore, and finally converts it to lowercase.
///
/// # Arguments
///
/// `str_to_format` - The string to format.
pub fn format_str(str_to_format: String) -> String {
    let mut ret = String::new();
    let mut count = 1;

    let re = regex::Regex::new(r"([A-Z][a-z]+)").expect("Unable to create regex pattern");
    for field in re.find_iter(&str_to_format) {
        if count > 1 {
            ret.push_str("_")
        }
        ret.push_str(&field.as_str().to_lowercase());
        count = count + 1;
    }

    return ret;
}

/// Returns the absolute path of a relative path.
///
/// # Arguments
///
/// `path` - The relative path to get the absolute path of.
pub fn absolute_path<P>(path: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    match dunce::canonicalize(&absolute_path) {
        Ok(v) => absolute_path = v,
        Err(_e) => {
            let parent = &absolute_path.parent().expect("Unable to parse path");
            let basename = &absolute_path.file_stem().expect("Unable to parse path");
            let parent_canon = dunce::canonicalize(parent).expect("Unable to parse path");

            absolute_path = parent_canon.join(basename);
        }
    }

    Ok(absolute_path)
}
