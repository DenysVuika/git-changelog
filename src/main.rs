use anyhow::Result;
use clap::Clap;

mod cli;
use cli::{Opts, SubCommand};

mod git;
use git::LogOptions;

mod formatting;
use formatting::{format_markdown, FormatOptions};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", &opts);

    // You can handle information about sub-commands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
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

    if let Some(remote) = git::get_remote(&opts.dir) {
        println!("remote: {}", remote);

        let commits = git::log(LogOptions {
            range: opts.range,
            dir: opts.dir,
            max_count: opts.max_count,
            skip: opts.skip,
        })?;

        format_markdown(FormatOptions {
            remote,
            commits,
            template: opts.template,
        })?;
    } else {
        println!("Remote not found");
    }

    Ok(())
}
