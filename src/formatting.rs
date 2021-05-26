use anyhow::Result;
use handlebars::{to_json, Handlebars};
use serde_json::json;
use std::path::PathBuf;

use crate::git::Commit;

#[derive(Debug)]
pub struct FormatOptions {
    pub remote: String,
    pub commits: Vec<Commit>,
    pub template: Option<PathBuf>,
}

pub fn format_markdown(options: FormatOptions) -> Result<()> {
    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);
    reg.register_template_file(
        "default",
        match &options.template {
            Some(template) => template.to_str().unwrap(),
            None => "./assets/templates/md.hbs",
        },
    )?;

    let mut repo_url = options.remote.clone();
    if repo_url.ends_with(".git") {
        repo_url.truncate(repo_url.len() - 4);
    }

    println!(
        "{}",
        reg.render(
            "default",
            &json!({
                "remote": options.remote,
                "repo_url": repo_url,
                "commits": to_json(options.commits)
            })
        )?
    );

    Ok(())
}
