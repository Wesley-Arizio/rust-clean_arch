use clap::Parser;
use cli::{Cli, Command};
use core_database::sqlite::DatabaseRepository;
mod create_organization;

pub mod cli;

use create_organization::create_organization;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();

    let db = DatabaseRepository::new()
        .await
        .map_err(|e| format!("Database error: {:#?}", e))?;

    match args.subcommand {
        Some(action) => match action {
            Command::CreateOrganization { name } => {
                let res = create_organization(&db, name).await?;

                println!(
                    "Organization '{}' was created successfuly with id: '{}'",
                    res.name, res.id
                );
            }
        },
        None => panic!("Select a valid subcommand"),
    };

    Ok(())
}
