use config::Config;

mod config;
mod creation;

pub mod cache;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new()?;

    creation::create_project(&config).await?;

    println!("Project successfully created!");

    Ok(())
}
