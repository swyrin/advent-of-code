use crate::options::options_generate::GenerateOptions;
use crate::utils::number::{pad_day_number, pad_year_number};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the current working directory.
pub fn get_cwd() -> String {
    let path = env::current_dir().expect("We are cooked");
    let path = path.to_str().expect("");

    String::from(path)
}

/// Get Advent of Code template path.
pub fn get_template_path() -> PathBuf {
    let root = get_cwd();

    Path::new(&root).join("crates").join("aoc_template")
}

/// Get Advent of Code submission path.
pub fn get_submission_path(opts: &GenerateOptions) -> PathBuf {
    let root = get_cwd();

    Path::new(&root)
        .join("advent_of_code")
        .join(format!("year_{}", pad_year_number(opts.get_year())))
        .join(format!("day_{}", pad_day_number(opts.get_day())))
}

/// Get capability to override.
/// Either the directory does not exist, or it exists and we can override it.
pub fn is_able_to_setup_submission(opts: &GenerateOptions) -> bool {
    let submission_path = get_submission_path(opts);
    !submission_path.exists() || opts.get_overwrite()
}

/// Run the submission directory creation via `cargo` command.
pub fn scaffold_submission_directory(opts: &GenerateOptions) {
    // sometimes I ask myself if rustup ever installs cargo.
    Command::new("cargo")
        .arg("init")
        .arg(get_submission_path(opts))
        .arg("--lib")
        .args([
            "--name",
            &format!("year_{}_day_{}", opts.get_year(), opts.get_day()),
        ])
        .args(["--vcs", "none"])
        .output()
        .expect("Unable to create cargo project.");
}

/// Teardown the submission directory. For testing. If clippy yells, blame the macro.
#[cfg(test)]
pub fn teardown_submission_directory(opts: &GenerateOptions) {
    use std::{fs, io};
    use toml_edit::DocumentMut;

    let cwd = get_cwd();
    let cwd = Path::new(cwd.as_str());

    let cargo_path = cwd.join("Cargo.toml");
    let source = fs::read_to_string(&cargo_path).unwrap();
    let mut doc = source.parse::<DocumentMut>().unwrap();

    let members = doc["workspace"]["members"]
        .as_array_mut()
        .ok_or_else(|| io::Error::other("workspace.members is not an array"))
        .unwrap();

    let nuke_pos = members
        .iter()
        .position(|value| {
            value.as_str()
                == Some(
                    format!(
                        "advent_of_code/year_{year}/day_{day}",
                        year = pad_year_number(opts.get_year()),
                        day = pad_day_number(opts.get_day())
                    )
                    .as_str(),
                )
        })
        .unwrap();

    members.remove(nuke_pos);

    fs::write(cargo_path, doc.to_string()).unwrap();

    let path = get_submission_path(opts);
    let path = Path::new(&path);
    fs::remove_dir_all(path.parent().unwrap()).expect("Unable to clear directory.");
}

#[cfg(test)]
mod test {
    use crate::options::options_generate::GenerateOptions;
    use crate::utils::fs::is_able_to_setup_submission;

    #[test]
    fn test_override_off_should_not_be_able_to_setup() {
        let opts = GenerateOptions::default();
        assert!(is_able_to_setup_submission(&opts));
    }

    #[test]
    fn test_override_on_should_be_able_to_setup() {
        let opts = GenerateOptions::new(2, 1, true);
        assert!(is_able_to_setup_submission(&opts));
    }
}
