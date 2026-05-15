/// Parse user supplied arguments
use std::{fmt::{Display, Formatter}, path::PathBuf};

use anyhow::Context;
use inquire::{CustomType, Select, Text};
use robostart::AllVariants;
use clap::{Parser, ValueEnum};

/// Which language to use
#[derive(Clone, Debug, ValueEnum, AllVariants)]
pub enum Language {
    Java,
    Cpp,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }    
}

/// Which type of project to initialize
#[derive(Clone, Debug, ValueEnum, AllVariants)]
pub enum ProjectType {
    Example,
    Template,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }    
}

#[derive(robostart::Parser, clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliParser {
    /// Language to initialize
    #[arg(short, long)]
    #[absent_handler(|_| Select::new(
        "Language: ",
        Language::all_variants().to_vec()
    )
        .prompt()
        .with_context(|| format!("Failed to prompt for language selection.")))]
    language: Option<Language>,

    /// Type of project to initalize
    #[arg(short, long)]
    #[absent_handler(|_| Select::new(
        "Project Type: ",
        ProjectType::all_variants().to_vec()
    )
        .prompt()
        .with_context(|| format!("Failed to prompt for the project type.")))]
    project_type: Option<ProjectType>,

    /// What version to download
    #[arg(short, long)]
    #[absent_handler(|parser: &Self| -> Result<String, anyhow::Error> {
        let proj_type = parser.project_type();
        Text::new(format!("{} Version: ", proj_type).as_str())
            .with_help_message("This will match the corresponding WPILib version, e.g. 2025.3.2")
            .prompt()
            .with_context(|| format!("Failed to prompt for WPILib version."))
    })]
    wpilib_version: Option<String>,

    /// The parent directory for the new project
    #[arg(short, long)]
    // idk why the compiler can't figure out that this returns an anyhow::Error
    #[absent_handler(|_| -> Result<_, anyhow::Error> {
        let prompt = Text::new("What directory should the project live under?")
            .with_help_message("This is just the parent directory of your project, don't include the project name.")
            .prompt()
            .with_context(|| "Failed to prompt for output prefix.")?;

        // grrr temporary value dropped while borrowed
        let home_dir = std::env::home_dir()
            .with_context(|| "Failed to read user\'s home directory into a `PathBuf`.")?;
        let home_dir = home_dir
            .to_str()
            .with_context(|| "Failed to read home directory path into a string.")?;

        let path_str = prompt.replace("~", home_dir); 

        Ok(PathBuf::from(path_str))
    })]
    output_prefix: Option<PathBuf>,

    /// Name of the new project
    #[arg(short, long)]
    #[absent_handler(|_| Text::new("Project Name: ")
        .prompt()
        .with_context(|| format!("Failed to prompt for project name.")))]
    name: Option<String>,

    /// Your team number
    #[arg(short, long)]
    #[absent_handler(|_| CustomType::new("Team Number: ")
        .with_formatter(&|i: u32| format!("{i}"))
        .with_error_message("Please type a valid integer greater than 0")
        .prompt()
        .with_context(|| format!("Could not read team number input.")))]
    team_number: Option<u32>,
}

impl CliParser {
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut ret = CliParser::parse();
        ret.handle_absent_values()?;
        Ok(ret)
    }
}
