use anyhow::Result;
use handlebars::{to_json, Handlebars};
use serde_json::json;
use std::{fs::File, path::PathBuf};

use crate::git::Commit;

#[derive(Debug)]
pub struct FormatOptions {
    pub remote: String,
    pub commits: Vec<Commit>,
    pub template: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

pub fn format_commits(options: FormatOptions) -> Result<()> {
    let mut reg = Handlebars::new();
    let template = "default";

    reg.set_strict_mode(true);
    reg.register_template_file(
        template,
        match &options.template {
            Some(template) => template.to_str().unwrap(),
            None => "./assets/templates/md.hbs",
        },
    )?;

    let mut repo_url = options.remote.clone();
    if repo_url.ends_with(".git") {
        repo_url.truncate(repo_url.len() - 4);
    }

    let data = &json!({
        "remote": options.remote,
        "repo_url": repo_url,
        "commits": to_json(options.commits)
    });

    match options.output {
        Some(output_path) => {
            let mut output_file = File::create(&output_path)?;
            reg.render_to_write(template, &data, &mut output_file)?;

            println!("Generated output to {}", &output_path.display().to_string());
        }
        None => {
            println!("{}", reg.render(template, data)?);
        }
    }

    Ok(())
}
