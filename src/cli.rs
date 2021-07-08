use clap::{AppSettings, Clap};
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(
    version = "1.0.0",
    author = "Denys Vuika <denys.vuika@gmail.com>",
    about = "Release changelog generator (https://github.com/DenysVuika/git-changelog)"
)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Sets a custom config file
    #[clap(short, long, default_value = "default.conf", value_name = "path")]
    pub config: String,

    /// Working directory
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = ".",
        value_name = "path"
    )]
    pub dir: PathBuf,

    /// Verbose output
    #[clap(short, long)]
    pub verbose: bool,

    // #[clap(subcommand)]
    // pub subcmd: Option<SubCommand>,
    /// Limit the number of commits to output
    #[clap(short = 'n', long, value_name = "number")]
    pub max_count: Option<i32>,

    /// Skip number commits before starting to show the commit output.
    #[clap(long, value_name = "number")]
    pub skip: Option<i32>,

    /// Path to the custom output template
    #[clap(short, long, parse(from_os_str))]
    pub template: Option<PathBuf>,

    /// Output file, will use console output if not defined
    #[clap(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,

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
