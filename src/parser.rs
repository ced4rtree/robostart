/// Parse user supplied arguments
use std::{fmt::{Display, Formatter}, path::PathBuf};

use inquire::{CustomType, InquireError, Select, Text};
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
    #[absent_handler(Select::new(
        "Language: ",
        Language::all_variants().to_vec()
    ).prompt().unwrap())]
    language: Option<Language>,

    /// Type of project to initalize
    #[arg(short, long)]
    #[absent_handler(Select::new(
        "Project Type: ",
        ProjectType::all_variants().to_vec()
    ).prompt().unwrap())]
    project_type: Option<ProjectType>,

    /// What version to download
    #[arg(short, long)]
    #[absent_handler({
        let proj_type = self.project_type();
        Text::new(
            format!("{} Version: ", proj_type).as_str()
        ).with_help_message("This will match the corresponding WPILib version, e.g. 2025.3.2")
        .prompt()
        .unwrap()
    })]
    wpilib_version: Option<String>,

    /// The parent directory for the new project
    #[arg(short, long)]
    #[absent_handler(PathBuf::from(Text::new(
        "What directory should the project live under?"
    ).with_help_message("This is just the parent directory of your project, don't include the project name.")
        .prompt()
        .unwrap()
        .replace("~", std::env::home_dir().unwrap().to_str().unwrap())))]
    output_prefix: Option<PathBuf>,

    /// Name of the new project
    #[arg(short, long)]
    #[absent_handler(Text::new("Project Name: ").prompt().unwrap())]
    name: Option<String>,

    /// Your team number
    #[arg(short, long)]
    #[absent_handler(CustomType::new("Team Number: ")
        .with_formatter(&|i: u32| format!("{i}"))
        .with_error_message("Please type a valid integer greater than 0")
        .prompt()
        .unwrap())]
    team_number: Option<u32>,
}

impl CliParser {
    pub fn new() -> Result<Self, InquireError> {
        let mut ret = CliParser::parse();
        ret.handle_absent_values();
        Ok(ret)
    }
}
