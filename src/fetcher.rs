use std::{fs::File, io};

use crate::{get_project_cache, get_project_file_path, parser::{CliParser, Language}};

pub async fn fetch_project(parser: &CliParser) -> Result<(), Box<dyn std::error::Error>> {
    let lang = match parser.language() {
        Language::Java => "wpilibj",
        Language::Cpp => "wpilibc",
    };

    let mut project_type = parser.project_type()
        .to_string()
        .to_lowercase();
    project_type.push('s');
    
    let url = format!(
        "https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{}/{}/{}/{}-{}.zip",
        lang,
        project_type,
        parser.wpilib_version(),
        project_type,
        parser.wpilib_version(),
    );

    std::fs::create_dir_all(get_project_cache())
        .expect(format!("Failed to create directory {}", get_project_cache().display()).as_str());

    // fetch zip file from artifactory, avoid downloading cached files
    let file_path = get_project_file_path(&parser);
    if !file_path.exists() {
        println!("Downloading {} as {}...", url, file_path.display());
        let resp = reqwest::get(url.as_str())
            .await
            .expect(format!("Failed to retrieve url: {url}").as_str());
        let mut out = File::create(file_path)
            .expect(format!("Failed to initialize {project_type} file on local filesystem").as_str());
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .expect(format!("Failed to populate {project_type} file on local filesystem").as_str());
    } else {
        println!("{} cached, skipping download.", file_path.display());
    }

    Ok(())
}
