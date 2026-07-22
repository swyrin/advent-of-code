use std::{fs, io, path::Path};

// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Get the current working directory.
pub fn get_cwd() -> String {
    let path = std::env::current_dir().expect("We are cooked");
    let path = path.to_str().expect("");

    String::from(path)
}
