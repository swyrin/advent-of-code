use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::prelude::*;
use clap::Args;
use std::process::Command;

use crate::utils;

#[derive(Args)]
pub struct GenerateOptions {
    /// The year. Default to this year.
    #[arg(short, long, default_value_t = Local::now().year() as usize)]
    pub year: usize,

    /// The day. Default to day 1.
    #[arg(short, long, default_value_t = 1)]
    pub day: usize,

    /// Whether to overwrite.
    #[arg(long, default_value_t = false)]
    pub overwrite: bool,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            year: Local::now().year() as usize,
            day: 1,
            overwrite: false,
        }
    }
}

/// Get submission files path.
pub fn get_submission_path(root: PathBuf, year: usize, day: usize) -> PathBuf {
    Path::new(&root)
        .join("advent_of_code")
        .join(format!("year_{}", year))
        .join(format!("day_{}", day))
}

/// Get template files path.
pub fn get_template_path(root: PathBuf) -> PathBuf {
    Path::new(&root).join("crates").join("aoc_template")
}

/// Get capability to override.
/// Either the directory does not exist, or it exists and we can override it.
pub fn is_able_to_setup_submission(path: PathBuf, override_flag: bool) -> bool {
    !path.exists() || override_flag
}

fn append_cargo_deps(submission_path: &Path) {
    let cargo_file = submission_path.join("Cargo.toml");

    let mut file = OpenOptions::new().append(true).open(cargo_file).unwrap();

    file.write_all(b"aoc_libraries.workspace = true\n")
        .expect("Unable to add workspace dep: aoc_libraries");

    file.write_all(b"aoc_macros.workspace = true\n")
        .expect("Unable to add workspace dep: aoc_macros");
}

pub fn generate(opts: &GenerateOptions) {
    let cwd = PathBuf::from_str(&utils::fs::get_cwd()).expect("smh getting cwd broke.");

    let template_source_path = get_template_path(cwd.clone());

    if !template_source_path.exists() {
        panic!(
            "No template path! There should be one at {}",
            String::from(template_source_path.to_str().unwrap())
        );
    }

    let submission_path = get_submission_path(cwd, opts.year, opts.day);

    if is_able_to_setup_submission(submission_path.clone(), opts.overwrite) {
        // sometimes I ask myself if rustup ever installs cargo.
        Command::new("cargo")
            .arg("init")
            .arg(&submission_path)
            .arg("--lib")
            .args(["--name", &format!("year_{}_day_{}", opts.year, opts.day)])
            .args(["--vcs", "none"])
            .output()
            .expect("Unable to create cargo project.");

        append_cargo_deps(&submission_path);
    } else {
        panic!(
            "We do not have the permit to setup submission, maybe --overwrite flag is not supplied?"
        );
    }
}
