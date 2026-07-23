use std::env;

/// Get the current working directory.
pub fn get_cwd() -> String {
    let path = env::current_dir().expect("We are cooked");
    let path = path.to_str().expect("");

    String::from(path)
}
