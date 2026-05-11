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
    language: Option<Language>,

    /// Type of project to initalize
    #[arg(short, long)]
    project_type: Option<ProjectType>,

    /// What version to download
    #[arg(short, long)]
    wpilib_version: Option<String>,

    /// The parent directory for the new project
    #[arg(short, long)]
    output_prefix: Option<PathBuf>,

    /// Name of the new project
    #[arg(short, long)]
    name: Option<String>,

    /// Your team number
    #[arg(short, long)]
    team_number: Option<u32>,
}

impl CliParser {
    pub fn new() -> Result<Self, InquireError> {
        let mut ret = CliParser::parse();

        if ret.language.is_none() {
            ret.language = Some(Select::new(
                "Language: ",
                Language::all_variants().to_vec()
            ).prompt()?);
        }

        if ret.project_type.is_none() {
            ret.project_type = Some(Select::new(
                "Project Type: ",
                ProjectType::all_variants().to_vec()
            ).prompt()?);
        }

        if ret.wpilib_version.is_none() {
            let proj_type = ret.project_type
                .as_ref()
                .unwrap();
            ret.wpilib_version = Some(Text::new(
                format!("{} Version: ", proj_type).as_str()
            ).with_help_message("This will match the corresponding WPILib version, e.g. 2025.3.2")
                .prompt()?);
        }

        if ret.output_prefix.is_none() {
            ret.output_prefix = Some(PathBuf::from(Text::new(
                "What directory should the project live under?"
            ).with_help_message("This is just the parent directory of your project, don't include the project name.")
                .prompt()?
                .replace("~", std::env::home_dir().unwrap().to_str().unwrap())))
        }

        if ret.name.is_none() {
            ret.name = Some(Text::new("Project Name: ")
                .prompt()?);
        }

        if ret.team_number.is_none() {
            ret.team_number = Some(CustomType::new("Team Number: ")
                .with_formatter(&|i: u32| format!("{i}"))
                .with_error_message("Please type a valid integer greater than 0")
                .prompt()?)
        }

        Ok(ret)
    }
}
