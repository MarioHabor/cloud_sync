use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn build_cli() -> Command {
    Command::new("cloud_sync")
        .about("Upload and sync files to multiple storage cloud providers")
        .arg(
            Arg::new("upload")
                .long("upload")
                .short('u')
                .action(ArgAction::SetTrue)
                .help("Upload command"),
        )
        .arg(
            Arg::new("check")
                .long("check")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Check command"),
        )
        .arg(
            Arg::new("generator")
                .long("generate")
                .short('g')
                .help("Generate shell completions")
                .value_parser(value_parser!(Shell)),
        )
}
pub fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
