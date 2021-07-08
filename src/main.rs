use anyhow::Result;
use clap::Clap;

mod cli;
use cli::Opts;

mod git;
use git::LogOptions;

mod formatting;
use formatting::{format_commits, FormatOptions};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // You can handle information about sub-commands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    /*
    if let Some(opts) = opts.subcmd {
        match opts {
            SubCommand::Test(t) => {
                if t.debug {
                    println!("Printing debug info");
                } else {
                    println!("Printing normally");
                }
            }
        }
    }
    */

    if let Some(remote) = git::get_remote(&opts.dir) {
        let commits = git::log(LogOptions {
            range: opts.range,
            dir: opts.dir,
            max_count: opts.max_count,
            skip: opts.skip,
        })?;

        format_commits(FormatOptions {
            remote,
            commits,
            template: opts.template,
            output: opts.output,
        })?;
    } else {
        println!("Error: Remote not found");
    }

    Ok(())
}
