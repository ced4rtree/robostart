use std::{fs::File, io, path::Path};

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Language {
    Java,
    Cpp,
}

#[derive(Clone, Debug, ValueEnum)]
enum ProjectType {
    Example,
    Template,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Language to initialize
    #[arg(short, long)]
    language: Language,

    /// Type of project to initalize
    #[arg(short = 't', long = "type")]
    project_type: ProjectType,

    /// What version to download
    #[arg(short, long)]
    wpilib_version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // parse arguments to figure out what file to fetch
    let lang = match args.language {
        Language::Java => "wpilibj",
        Language::Cpp  => "wpilibc",
    };

    let project_type = match args.project_type {
        ProjectType::Example  => "example",
        ProjectType::Template => "template",
    };

    let file_name = format!("{lang}-{project_type}-{}.zip", args.wpilib_version);

    let url = format!(
        "https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{}/{}/{}/templates/{}.zip",
        lang,
        project_type,
        args.wpilib_version,
        args.wpilib_version,
    );

    let file_cache = format!("{}/.cache/robostart/", std::env::home_dir().unwrap().display());
    std::fs::create_dir_all(file_cache.as_str())
        .expect(format!("Failed to create directory {}", file_cache).as_str());

    // fetch zip file from artifactory, avoid downloading cached files
    if !Path::new(format!("{}{}", file_cache, file_name).as_str()).exists() {
        let resp = reqwest::get(url.as_str())
            .await
            .expect(format!("Failed to retrieve url: {url}").as_str());
        let mut out = File::create(format!("{}{}", file_cache, file_name))
            .expect(format!("Failed to initialize {project_type} file on local filesystem").as_str());
        io::copy(&mut resp.bytes().await?.as_ref(), &mut out)
            .expect(format!("Failed to populate {project_type} file on local filesystem").as_str());
    }

    Ok(())
}
