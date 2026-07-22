mod commands;
mod utils;

use crate::commands::generate::{GenerateOptions, generate};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate the template.
    Generate(GenerateOptions),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(args) => generate(args),
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::PathBuf, str::FromStr};

    use crate::{
        commands::generate::{
            GenerateOptions, generate, get_submission_path, is_able_to_setup_submission,
        },
        utils::fs::get_cwd,
    };

    #[test]
    fn test_override_off() {
        let opts = GenerateOptions::default();

        let path = get_submission_path(PathBuf::from_str(&get_cwd()).unwrap(), opts.year, opts.day);

        generate(&opts);
        assert!(path.exists());
        assert!(!is_able_to_setup_submission(path.clone(), opts.overwrite));

        let parent = path.parent().unwrap();
        fs::remove_dir_all(parent).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn test_override_on() {
        let opts = GenerateOptions {
            year: 2,
            day: 1,
            overwrite: true,
        };

        let path = get_submission_path(PathBuf::from_str(&get_cwd()).unwrap(), opts.year, opts.day);

        generate(&opts);
        assert!(path.exists());
        assert!(is_able_to_setup_submission(path.clone(), opts.overwrite));

        let parent = path.parent().unwrap();
        fs::remove_dir_all(parent).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn test_override_before_creation_is_on() {
        let opts = GenerateOptions {
            year: 2,
            day: 1,
            overwrite: true,
        };

        let path = get_submission_path(PathBuf::from_str(&get_cwd()).unwrap(), opts.year, opts.day);

        assert!(!path.exists());
        assert!(is_able_to_setup_submission(path.clone(), opts.overwrite));
    }
}
