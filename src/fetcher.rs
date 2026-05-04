use std::{fs::File, io};

use crate::{get_cached_commands_vendordep, get_cached_gitignore, get_project_zip_path, get_robostart_cache, parser::CliParser};

pub async fn fetch_project(parser: &CliParser) -> Result<(), Box<dyn std::error::Error>> {
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

    std::fs::create_dir_all(get_robostart_cache())
        .unwrap_or_else(|_| panic!("Failed to create directory {:?}", get_robostart_cache()));

    // fetch zip file from github, avoid downloading cached files
    let zip_file_path = get_project_zip_path(parser);
    if !zip_file_path.exists() {
        println!("Downloading {} as {}...", zip_url, zip_file_path.display());
        let resp = reqwest::get(zip_url.as_str())
            .await
            .unwrap_or_else(|_| panic!("Failed to retrieve url: {zip_url}"));
        let mut out = File::create(zip_file_path)
            .unwrap_or_else(|_| panic!("Failed to initialize {project_type} file on local filesystem"));
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .unwrap_or_else(|_| panic!("Failed to populate {project_type} file on local filesystem"));
    } else {
        println!("{} cached, skipping download.", zip_file_path.display());
    }

    // fetch gitignore
    let gitignore_file = get_cached_gitignore();
    let gitignore_url = "https://raw.githubusercontent.com/wpilibsuite/vscode-wpilib/refs/heads/main/vscode-wpilib/resources/gradle/shared/.gitignore";
    if !gitignore_file.exists() {
        println!("Downloading {} as {}...", gitignore_url, gitignore_file.display());
        let resp = reqwest::get(gitignore_url)
            .await
            .unwrap_or_else(|_| panic!("Failed to retrieve url: {gitignore_url}"));
        let mut out = File::create(&gitignore_file)
            .unwrap_or_else(|_| panic!("Failed to create file {:?}", gitignore_file));
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .unwrap_or_else(|_| panic!("Failed to populate {:?} file on local filesystem", gitignore_file));
    } else {
        println!("{} cached, skipping download.", gitignore_file.display());
    }

    // fetch vendordeps/WPILibNewCommands.json
    let commands_file = get_cached_commands_vendordep(parser);
    std::fs::create_dir_all(commands_file.parent().unwrap())?;
    if !commands_file.exists() {
        let commands_url = format!(
            "https://raw.githubusercontent.com/wpilibsuite/allwpilib/refs/tags/v{}/wpilibNewCommands/WPILibNewCommands.json",
            parser.wpilib_version()
        );
        println!("Downloading {commands_url} as {:?}...", commands_file);

        let resp = reqwest::get(&commands_url)
            .await
            .unwrap_or_else(|_| panic!("Failed to retrieve url: {commands_url}"));
        let mut out = File::create(&commands_file)
            .unwrap_or_else(|_| panic!("Failed to create file {:?}", commands_file));
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .unwrap_or_else(|_| panic!("Failed to populate {:?} with data", out));
    } else {
        println!("{:?} already cached, skipping download.", commands_file);
    }

    Ok(())
}
