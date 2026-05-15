use std::{fs::File, io};

use anyhow::Context;

use crate::{get_cached_commands_vendordep, get_cached_gitignore, get_project_zip_path, get_robostart_cache, parser::CliParser};

pub async fn fetch_project(parser: &CliParser) -> anyhow::Result<()> {
    // transform e.g. "Template" into "templates"
    let mut project_type = parser.project_type()
        .to_string()
        .to_lowercase();
    project_type.push('s');
    
    let zip_url = format!(
        "https://github.com/wpilibsuite/vscode-wpilib/releases/download/v{}/{}.zip",
        parser.wpilib_version(),
        project_type,
    );

    let robostart_cache = &get_robostart_cache();
    std::fs::create_dir_all(robostart_cache)
        .with_context(|| format!("Failed to create directory {:?}", robostart_cache))?;

    // fetch zip file from github, avoid downloading cached files
    let zip_file_path = get_project_zip_path(parser);
    if !zip_file_path.exists() {
        println!("Downloading {} as {}...", zip_url, zip_file_path.display());
        let resp = reqwest::get(zip_url.as_str())
            .await
            .with_context(|| format!("Failed to retrieve url: {zip_url}"))?;
        let mut out = File::create(zip_file_path)
            .with_context(|| format!("Failed to initialize {project_type} file on local filesystem"))?;
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .with_context(|| format!("Failed to populate {project_type} file on local filesystem"))?;
    } else {
        println!("{:?} cached, skipping download.", zip_file_path);
    }

    // fetch gitignore
    let gitignore_file = get_cached_gitignore();
    let gitignore_url = "https://raw.githubusercontent.com/wpilibsuite/vscode-wpilib/refs/heads/main/vscode-wpilib/resources/gradle/shared/.gitignore";
    if !gitignore_file.exists() {
        println!("Downloading {} as {}...", gitignore_url, gitignore_file.display());
        let resp = reqwest::get(gitignore_url)
            .await
            .with_context(|| format!("Failed to retrieve url: {gitignore_url}"))?;
        let mut out = File::create(&gitignore_file)
            .with_context(|| format!("Failed to create file {:?}", gitignore_file))?;
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .with_context(|| format!("Failed to populate {:?} file on local filesystem", gitignore_file))?;
    } else {
        println!("{} cached, skipping download.", gitignore_file.display());
    }

    // fetch vendordeps/WPILibNewCommands.json
    let commands_file = get_cached_commands_vendordep(parser);
    let commands_file_parent = commands_file.parent()
        .with_context(|| format!("Failed to retrieve parent of file: {:?}", commands_file))?; 
    std::fs::create_dir_all(commands_file_parent)?;
    if !commands_file.exists() {
        let commands_url = format!(
            "https://raw.githubusercontent.com/wpilibsuite/allwpilib/refs/tags/v{}/wpilibNewCommands/WPILibNewCommands.json",
            parser.wpilib_version()
        );
        println!("Downloading {commands_url} as {:?}...", commands_file);

        let resp = reqwest::get(&commands_url)
            .await
            .with_context(|| format!("Failed to retrieve url: {commands_url}"))?;
        let mut out = File::create(&commands_file)
            .with_context(|| format!("Failed to create file {:?}", commands_file))?;
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .with_context(|| format!("Failed to populate {:?} with data", out))?;
    } else {
        println!("{:?} already cached, skipping download.", commands_file);
    }

    Ok(())
}
