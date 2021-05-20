use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "1.0.0", author = "Denys Vuika <denys.vuika@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    pub config: String,

    /// Working directory
    #[clap(short, long, parse(from_os_str), default_value = ".")]
    pub dir: PathBuf,

    /// Some input. Because this isn't an Option<T> it's required to be used
    // input: String,

    /// Verbose output
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Clap)]
pub enum SubCommand {
    /// Sub-command
    #[clap(version = "1.0", author = "Denys Vuika <denys.vuika@gmail.com>")]
    Test(Test),
}

#[derive(Clap)]
pub struct Test {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
}
