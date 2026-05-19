use config::Config;

use crate::config::Action;

mod config;
mod creation;

pub mod cache;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new()?;

    match config.action()? {
        Action::Create => creation::create_project(&config).await?,
        Action::Import => println!("Thanks for importing byee"),
    };

    Ok(())
}
