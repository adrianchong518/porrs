use std::path;
use std::process::exit;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum ExecutionMode {
    /// Simulate the provided program
    #[clap(name = "sim")]
    Simulate,

    /// Compile the provided program to binary (Not implemented)
    #[clap(name = "com")]
    NativeCompile,
}

#[derive(Debug, Parser)]
#[clap(version)]
#[clap(about = "Porth compiler / simulator in Rust", long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Config {
    /// Porth source file
    #[clap(parse(from_os_str))]
    pub source_file: path::PathBuf,

    #[clap(subcommand)]
    pub execution_mode: ExecutionMode,
}

fn init_logger() {
    use env_logger::{Builder, Env};

    let env = Env::default()
        .filter_or("PORRS_LOG", "info")
        .write_style_or("PORRS_LOG_STYLE", "always");

    Builder::from_env(env).format_timestamp(None).init();
}

fn run(config: &Config) -> Result<(), porrs::Error> {
    let program = porrs::Program::from_path(&config.source_file)?;

    match config.execution_mode {
        ExecutionMode::Simulate => porrs::simulate(&program),
        ExecutionMode::NativeCompile => unimplemented!("File compilation is not yet implemented"),
    }
}

fn main() {
    init_logger();

    let config = Config::parse();
    log::debug!("CLI Config: {:#?}", config);

    if let Err(err) = run(&config) {
        eprintln!("ERROR | {}", err);
        for info in err.info_stack() {
            eprintln!("NOTE  | {}", info)
        }

        exit(1);
    }
}
