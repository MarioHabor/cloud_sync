pub mod cache;
pub mod cli;
pub mod commands;
pub mod operation;
pub mod toml;

use clap_complete::Shell;

fn main() {
    let matches = cli::build_cli().get_matches();

    if matches.get_flag("upload") {
        let _ = operation::begin_upload();
    }

    if matches.get_flag("check") {
        let _ = operation::check_last_update();
    }

    if let Some(generator) = matches.get_one::<Shell>("generator") {
        let mut cmd = cli::build_cli();
        eprintln!("Generating completion file for {generator}...");
        cli::print_completions(*generator, &mut cmd);
    }
}
