mod merge;
mod update;

use std::{env::current_dir, fs, path::{self, Path}, process};
use clap::{Parser, Subcommand};

pub const SRC_DIR: &str = "src";
pub const LIB_DIR: &str = "lib";
pub const OUT_DIR: &str = "scripts";

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Optional path to project. If left empty uses current working directory
    #[arg(long, short)]
    path: Option<path::PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Creates the src and lib directories, copying any scripts into the src directory
    New {},

    /// Default if no command specified. Merges library files into src files in the scripts directory
    Merge {},

    /// Checks to see if any new scripts have been created, and copies their contents to the src directory
    Update {}
}

fn main() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(p) => p,
        None => current_dir().expect("Could not get current working directory.")
    };

    if !Path::exists(&path) {
        eprintln!("Error: Source path does not exist.");
        process::exit(1);
    }
    if path.is_file() {
        eprintln!("Error: Source path is not a directory.");
        process::exit(1);
    }

    match &cli.command {
        Some(Commands::New {  }) => {
            match fs::create_dir(path.join(SRC_DIR)) {
                Ok(_) => {},
                Err(error) => eprintln!("Error creating src directory: {error}"),
            };
            match fs::create_dir(path.join(LIB_DIR)) {
                Ok(_) => {},
                Err(error) => eprintln!("Error creating lib directory: {error}"),
            };

            match update::run(path) {
                Ok(count) => println!("Added {count} new files to src directory."),
                Err(error) => {
                    eprintln!("Error: {error}");
                    process::exit(1);
                }
            };
        }
        Some(Commands::Merge {  }) | None => {
            println!("Merging...");
        
            match merge::run(path) {
                Ok(count) => println!("Successfully merged {count} file(s)."),
                Err(error) => {
                    eprintln!("Error: {error}");
                    process::exit(1);
                }
            };
        }
        Some(Commands::Update {  }) => {
            println!("Updating...");

            match update::run(path) {
                Ok(count) => println!("Added {count} new files to src directory."),
                Err(error) => {
                    eprintln!("Error: {error}");
                    process::exit(1);
                }
            };
        }
    }
}
