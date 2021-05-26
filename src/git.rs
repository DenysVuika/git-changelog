use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

// #[derive(PartialEq, Default, Clone, Debug)]
// pub struct Commit {
//     pub hash: String,
//     pub message: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub author_email: String,
    pub date: String,
    pub subject: String,
}

#[derive(Debug)]
pub struct LogOptions {
    pub range: String,
    pub dir: PathBuf,
    pub max_count: Option<i32>,
    pub skip: Option<i32>,
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

pub fn log(options: LogOptions) -> Result<Vec<Commit>> {
    println!("{:?}", &options);

    let args: Vec<&str> = [
        "log",
        "--no-merges",
        "--first-parent",
        "--invert-grep",
        "--author=bot\\|Alfresco Build User",
        "--format={ ^@^hash^@^: ^@^%h^@^, ^@^author^@^: ^@^%an^@^, ^@^author_email^@^: ^@^%ae^@^, ^@^date^@^: ^@^%ad^@^, ^@^subject^@^: ^@^%s^@^ }",
        &options.range,
    ].to_vec();

    let mut command = Command::new("git");
    command.args(&args).current_dir(&options.dir);

    if let Some(max_count) = &options.max_count {
        command.arg(format!("--max-count={}", max_count));
    }

    if let Some(skip) = &options.skip {
        command.arg(format!("--skip={}", skip));
    }

    let output = command.output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr).to_string());
    }

    let commits = String::from_utf8(output.stdout)?
        .lines()
        .map(|json| {
            // https://stackoverflow.com/a/13928240/14644447
            let json = str::replace(&json, "\"", "\\\"");
            let json = str::replace(&json, "^@^", "\"");
            serde_json::from_str(&json).unwrap()
        })
        .collect();

    Ok(commits)
}
