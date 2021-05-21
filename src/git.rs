use anyhow::Result;
use serde::Deserialize;
use std::{path::PathBuf, process::Command};

// #[derive(PartialEq, Default, Clone, Debug)]
// pub struct Commit {
//     pub hash: String,
//     pub message: String,
// }

#[derive(Debug, Deserialize)]
struct Commit {
    commit: String,
    author: String,
    author_email: String,
    date: String,
    subject: String,
}

pub fn get_remote(dir: &PathBuf) -> Option<String> {
    if dir.exists() {
        let output = Command::new("git")
            .args(&["config", "--get", "remote.origin.url"])
            .current_dir(dir)
            .output()
            .expect("Get remote command failed");

        if output.status.success() {
            let mut remote = String::from_utf8_lossy(&output.stdout).to_string();

            let len = remote.trim_end_matches(&['\r', '\n'][..]).len();
            remote.truncate(len);

            /*
            if remote.ends_with(".git") {
                remote.truncate(remote.len() - 4);
            }
            */

            return Some(remote);
        }
    }
    None
}

pub fn log(dir: &PathBuf) -> Result<()> {
    let output = Command::new("git")
        .args(&[
            "log",
            "master..develop",
            "--no-merges",
            "--first-parent",
            "--invert-grep",
            "--author=bot\\|Alfresco Build User",
            "--format={ \"commit\": \"%h\", \"author\": \"%an\", \"author_email\": \"%ae\", \"date\": \"%ad\", \"subject\": \"%s\" }",
        ])
        .current_dir(dir)
        .output()?;

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr).to_string());
        panic!("Command executed with failing error code");
    }

    String::from_utf8(output.stdout)?
        .lines()
        .take(10)
        .map(|json| serde_json::from_str(json).unwrap())
        .for_each(|x: Commit| println!("{:?}", x));

    Ok(())
}