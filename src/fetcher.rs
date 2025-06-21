use std::{fs::File, io};

use clap::ValueEnum;

use crate::{get_project_cache, get_project_file_name, get_project_file_path, Args};

/// Which language to use
#[derive(Clone, Debug, ValueEnum)]
pub enum Language {
    Java,
    Cpp,
}

/// Which type of project to initialize
#[derive(Clone, Debug, ValueEnum)]
pub enum ProjectType {
    Example,
    Template,
}

pub async fn fetch_project(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments to figure out what file to fetch
    let lang = match args.language {
        Language::Java => "wpilibj",
        Language::Cpp  => "wpilibc",
    };

    let project_type = match args.project_type {
        ProjectType::Example  => "examples",
        ProjectType::Template => "templates",
    };

    let url = format!(
        "https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{}/{}/{}/{}-{}.zip",
        lang,
        project_type,
        args.wpilib_version,
        project_type,
        args.wpilib_version,
    );
    

    std::fs::create_dir_all(get_project_cache())
        .expect(format!("Failed to create directory {}", get_project_cache().display()).as_str());

    // fetch zip file from artifactory, avoid downloading cached files
    let file_path = get_project_file_path(&args);
    if !file_path.exists() {
        let resp = reqwest::get(url.as_str())
            .await
            .expect(format!("Failed to retrieve url: {url}").as_str());
        let mut out = File::create(file_path)
            .expect(format!("Failed to initialize {project_type} file on local filesystem").as_str());
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .expect(format!("Failed to populate {project_type} file on local filesystem").as_str());
    } else {
        println!(
            "{} cached, skipping download.",
            get_project_file_name(&args).display()
        );
    }

    Ok(())
}
