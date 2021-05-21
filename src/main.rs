use anyhow::Result;
use clap::Clap;

mod cli;
use cli::{Opts, SubCommand};

mod git;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    println!("Value for config: {}", opts.config);
    println!("Value for dir: {:?}", opts.dir);
    println!("Verbose output: {}", opts.verbose);

    // You can handle information about subcommands by requesting their matches by name
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
        git::log(&opts.dir)?;
    } else {
        println!("Remote not found");
    }

    Ok(())
}
