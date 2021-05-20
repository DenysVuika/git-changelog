use std::{path::PathBuf, process::Command};

pub fn get_remote(dir: PathBuf) -> Option<String> {
    if dir.exists() {
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

            return Some(remote);
        }
    }
    None
}
