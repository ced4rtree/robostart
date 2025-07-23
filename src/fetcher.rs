use std::{fs::File, io};

use crate::{get_cached_commands_vendordep, get_project_zip_path, get_robostart_cache, parser::CliParser};

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
        .expect(format!("Failed to create directory {:?}", get_robostart_cache()).as_str());

    // fetch zip file from github, avoid downloading cached files
    let zip_file_path = get_project_zip_path(&parser);
    if !zip_file_path.exists() {
        println!("Downloading {} as {}...", zip_url, zip_file_path.display());
        let resp = reqwest::get(zip_url.as_str())
            .await
            .expect(format!("Failed to retrieve url: {zip_url}").as_str());
        let mut out = File::create(zip_file_path)
            .expect(format!("Failed to initialize {project_type} file on local filesystem").as_str());
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .expect(format!("Failed to populate {project_type} file on local filesystem").as_str());
    } else {
        println!("{} cached, skipping download.", zip_file_path.display());
    }

    // fetch vendordeps/WPILibNewCommands.json
    let commands_file = get_cached_commands_vendordep(&parser);
    std::fs::create_dir_all(commands_file.parent().unwrap())?;
    if !commands_file.exists() {
        let commands_url = format!(
            "https://raw.githubusercontent.com/wpilibsuite/allwpilib/refs/tags/v{}/wpilibNewCommands/WPILibNewCommands.json",
            parser.wpilib_version()
        );
        println!("Downloading {commands_url} as {:?}...", commands_file);

        let resp = reqwest::get(&commands_url)
            .await
            .expect(format!("Failed to retrieve url: {commands_url}").as_str());
        let mut out = File::create(&commands_file)
            .expect(format!("Failed to create file {:?}", commands_file).as_str());
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .expect(format!("Failed to populate {:?} with data", out).as_str());
    } else {
        println!("{:?} already cached, skipping download.", commands_file);
    }

    Ok(())
}
