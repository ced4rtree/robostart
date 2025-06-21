/// Parse user supplied arguments

use std::{fmt::{Display, Formatter}, path::PathBuf};

use inquire::{InquireError, Select, Text};
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

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Language to initialize
    #[arg(short, long)]
    language: Option<Language>,

    /// Type of project to initalize
    #[arg(short = 't', long = "type")]
    project_type: Option<ProjectType>,

    /// What version to download
    #[arg(short, long)]
    wpilib_version: Option<String>,

    /// Where to put the new project
    #[arg(short, long)]
    output_prefix: Option<PathBuf>,

    /// Name of the new project
    #[arg(short, long)]
    name: Option<String>,
}

pub struct CliParser {
    args: Args
}

impl CliParser {
    pub fn new() -> Result<Self, InquireError> {
        let mut ret = CliParser {
            args: Args::parse(),
        };

        if ret.args.language.is_none() {
            ret.args.language = Some(Select::new(
                "Language: ",
                Language::all_variants().to_vec()
            ).prompt()?);
        }

        if ret.args.project_type.is_none() {
            ret.args.project_type = Some(Select::new(
                "Project Type: ",
                ProjectType::all_variants().to_vec()
            ).prompt()?);
        }

        if ret.args.wpilib_version.is_none() {
            let proj_type = ret.args
                .project_type
                .as_ref()
                .unwrap();
            ret.args.wpilib_version = Some(Text::new(
                format!("{} Version: ", proj_type).as_str()
            ).with_help_message("This will match the corresponding WPILib version, e.g. 2025.3.2")
                .prompt()?);
        }

        if ret.args.output_prefix.is_none() {
            ret.args.output_prefix = Some(PathBuf::from(Text::new(
                "What directory should the project live under?"
            ).with_help_message("This is just the parent directory of your project, don't include the project name.")
                .prompt()?
                .replace("~", std::env::home_dir().unwrap().to_str().unwrap())))
        }

        if ret.args.name.is_none() {
            ret.args.name = Some(Text::new("Project Name: ")
                .prompt()?);
        }

        Ok(ret)
    }

    pub fn language(&self) -> Language {
        return self.args.language.clone().unwrap();
    }

    pub fn project_type(&self) -> ProjectType {
        return self.args.project_type.clone().unwrap();
    }

    pub fn wpilib_version(&self) -> String {
        return self.args.wpilib_version.clone().unwrap();
    }

    pub fn output_prefix(&self) -> PathBuf {
        return self.args.output_prefix.clone().unwrap();
    }

    pub fn name(&self) -> String {
        return self.args.name.clone().unwrap();
    }
}
