use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use clap::{arg, Arg, ArgAction, ArgMatches, command, Command, Error, value_parser, ValueHint};
use clap_complete::Shell;
use log::{error, info, LevelFilter, trace};
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};

fn cli_build() -> Command {
    Command::new("build")
        .about("Creates a new .gej file from a .blend file")
        .arg(Arg::new("file")
            .help("The .blend file to convert")
            .action(ArgAction::Append)
            .value_hint(ValueHint::FilePath))
}

fn cli_app() -> Command {
    Command::new("gentity-cli")
        .subcommand(cli_build())
}

#[derive(Debug)]
enum CliError {
    Unknown,
    BuildMissingFileSpec,
    BuildExpectedBlendFileFormat,
    BuildFailed(BlendTransformError),
    BuildFilePathError(io::Error),
}

fn main() -> Result<(), CliError> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    let matches = cli_app().get_matches_from(wild::args());

    if let Some(matches) = matches.subcommand_matches("build") {
        build(matches)?;
    }

    Ok(())
}

fn build(matches: &ArgMatches) -> Result<(), CliError> {
    let mut files = vec![];
    if let Some(builds) = matches.get_many::<String>("file") {
        for build in builds {
            let path = Path::new(build);
            let path = match std::fs::canonicalize(path) {
                Ok(path) => path,
                Err(e) => {
                    error!("Failed to canonicalize path {:?}: {:?}", path, e);
                    return Err(CliError::BuildFilePathError(e));
                }
            };
            files.push(path);
        }
    } else {
        error!("No files specified for build");
        return Err(CliError::BuildMissingFileSpec);
    }

    for file in files {
        trace!("Processing file: {:?}", file);
        match file.extension() {
            Some(ext) if ext == "blend" => match transform_blend_to_gej(&file) {
                Ok(_) => {}
                Err(e) => {
                    error!("Processing file {:?} failed: {:?}", file, e);
                    return Err(CliError::BuildFailed(e));
                }
            },
            _ => {
                error!("Unknown file type: {:?}", file);
                return Err(CliError::BuildExpectedBlendFileFormat);
            }
        }
    }
    Ok(())
}


#[test]
fn verify_cli() {
    cli_app().debug_assert();
}

#[test]
fn cli_app_no_args() {
    let app = cli_app();
    let matches = app.get_matches_from(vec!["gentity-cli"]);
    assert!(matches.subcommand_matches("build").is_none());
}

#[test]
fn cli_build_no_args() {
    let app = cli_app();
    let matches = app.get_matches_from(vec!["gentity-cli", "build"]);
    assert!(matches.subcommand_matches("build").is_some());
    assert!(matches.subcommand_matches("build").unwrap().get_many::<String>("file").is_none());
}

#[test]
fn build_with_one_file() {
    let app = cli_app();
    let matches = app.get_matches_from(vec!["gentity-cli", "build", "file.blend"]);
    assert_eq!(matches.subcommand_matches("build").unwrap().get_many::<String>("file").unwrap().collect::<Vec<&String>>(), vec!["file.blend"]);
}

#[test]
fn build_with_multiple_files() {
    let app = cli_app();
    let matches = app.get_matches_from(vec!["gentity-cli", "build", "file1.blend", "file2.blend"]);
    assert_eq!(matches.subcommand_matches("build").unwrap().get_many::<String>("file").unwrap().collect::<Vec<&String>>(), vec!["file1.blend", "file2.blend"]);
}