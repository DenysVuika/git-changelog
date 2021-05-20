use clap::Clap;
use std::path::PathBuf;
use std::process::Command;

mod cli;
use cli::{Opts, SubCommand};

fn get_remote(dir: PathBuf) -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .current_dir(dir)
        .output()
        .expect("ls command failed");

    if output.status.success() {
        let mut remote = String::from_utf8_lossy(&output.stdout).to_string();

        let len = remote.trim_end_matches(&['\r', '\n'][..]).len();
        remote.truncate(len);

        /*
        if remote.ends_with(".git") {
            remote.truncate(remote.len() - 4);
        }
        */

        Some(remote)
    } else {
        None
    }
}

fn main() {
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

    if let Some(remote) = get_remote(opts.dir) {
        println!("remote: {}", remote);
    } else {
        println!("Remote not found");
    }
}
