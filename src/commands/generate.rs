use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::options::options_generate::GenerateOptions;
use crate::utils::fs::{
    get_submission_path, get_template_path, is_able_to_setup_submission,
    scaffold_submission_directory,
};

/// Fetch the input for that day & year.
///
/// Requires session token, since the input is personalized.
/// See `crate::utils::session_token::get_session_token`
fn get_input_file_content(opts: &GenerateOptions) -> String {
    #[cfg(not(test))]
    {
        use crate::utils::session_token::get_session_token;
        use reqwest::header::HeaderMap;

        let complete_url = format!(
            "https://adventofcode.com/{year}/day/{day}/input",
            year = opts.get_year(),
            day = opts.get_day()
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Scrap-Author",
            "Tien Dat Pham <hello@swyrin.me>".parse().unwrap(),
        );
        headers.insert(
            "Cookie",
            format!("session={}", get_session_token()).parse().unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let body = client.get(complete_url).send();
        let body = body.unwrap();

        let error = format!(
            "No input for year {year} and day {day}",
            year = opts.get_year(),
            day = opts.get_day()
        );

        match body.status().is_success() {
            true => body.text().unwrap(),
            false => panic!("{}", error),
        }
    }

    #[cfg(test)]
    {
        // https://en.wikipedia.org/wiki/Advent_of_Code
        // "It has been running since 2015"
        match opts.get_year() >= 2015 {
            true => String::from("ligma"),
            false => panic!(),
        }
    }
}

fn append_cargo_deps(submission_path: &Path) {
    let cargo_file = submission_path.join("Cargo.toml");

    let mut file = OpenOptions::new().append(true).open(cargo_file).unwrap();

    file.write_all(b"aoc_libraries.workspace = true\n")
        .expect("Unable to add workspace dep: aoc_libraries");

    file.write_all(b"aoc_macros.workspace = true\n")
        .expect("Unable to add workspace dep: aoc_macros");
}

/// Copy in `<template>/src` into `<template>/src`.
fn mimic_src_file_from_template(template_path: &Path, submission_path: &Path, file_name: &str) {
    let template_file = template_path.join("src").join(file_name);
    let target_file = submission_path.join("src").join(file_name);

    let content = fs::read_to_string(template_file).expect("Template file went missing?");

    fs::write(target_file, content).expect("Unable to write from the template");
}

fn write_input(submission_path: &Path, opts: &GenerateOptions) {
    let input_path = submission_path.join("input.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(input_path)
        .unwrap();

    file.write_all(get_input_file_content(opts).as_bytes())
        .expect("Unable to write input.");
}

pub fn generate(opts: &GenerateOptions) {
    let template_source_path = get_template_path();

    if !template_source_path.exists() {
        panic!(
            "No template path! There should be one at {}",
            String::from(template_source_path.to_str().unwrap())
        );
    }

    let submission_path = get_submission_path(opts);

    if is_able_to_setup_submission(opts) {
        scaffold_submission_directory(opts);
        append_cargo_deps(&submission_path);
        mimic_src_file_from_template(&template_source_path, &submission_path, "lib.rs");
        mimic_src_file_from_template(&template_source_path, &submission_path, "input.rs");
        mimic_src_file_from_template(&template_source_path, &submission_path, "part_01.rs");
        mimic_src_file_from_template(&template_source_path, &submission_path, "part_02.rs");
        write_input(&submission_path, opts);
    } else {
        panic!(
            "We do not have the permit to setup submission, maybe --overwrite flag is not supplied?"
        );
    }
}

#[cfg(test)]
mod test {
    use crate::commands::generate::{generate, get_input_file_content};
    use crate::options::options_generate::GenerateOptions;
    use crate::utils::fs::{get_submission_path, teardown_submission_directory};

    #[test]
    fn test_input_retrieval_valid_yes_this_shit_is_mocked() {
        let opts = GenerateOptions::new(2025, 12, false);
        let input = get_input_file_content(&opts);

        assert_ne!(&input, "");
    }

    #[test]
    fn test_generate_should_have_enough_files() {
        let opts = GenerateOptions::default();

        generate(&opts);

        let submission_path = get_submission_path(&opts);
        let source_path = submission_path.join("src");

        assert!(source_path.join("lib.rs").exists());
        assert!(source_path.join("input.rs").exists());
        assert!(source_path.join("part_01.rs").exists());
        assert!(source_path.join("part_02.rs").exists());

        teardown_submission_directory(&opts);
    }

    #[test]
    #[should_panic]
    fn test_input_retrieval_invalid() {
        let opts = GenerateOptions::new(1, 1, false);
        get_input_file_content(&opts);
    }
}
