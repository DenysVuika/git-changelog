use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(version = "1.0.0", author = "Denys Vuika <denys.vuika@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    pub config: String,

    /// Working directory
    #[clap(short, long, parse(from_os_str), default_value = ".")]
    pub dir: PathBuf,

    /// Verbose output
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,

    /// Commit range, i.e. master..develop
    pub range: String,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Sub-command
    #[clap(version = "1.0", author = "Denys Vuika <denys.vuika@gmail.com>")]
    Test(Test),
}

#[derive(Clap, Debug)]
pub struct Test {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
}
