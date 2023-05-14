use clap::{Parser, ValueEnum};
use core_database::sqlite::DatabaseRepository;
mod create_organization;

use create_organization::create_organization;

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Actions {
    CreateOrganization,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the company
    #[arg(short, long)]
    name: String,

    /// Action to be called
    #[clap(long, value_enum, value_parser)]
    action: Actions,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    let db = DatabaseRepository::new()
        .await
        .map_err(|e| format!("Database error: {:#?}", e))?;

    match args.action {
        Actions::CreateOrganization => {
            let res = create_organization(&db, args.name).await?;

            println!(
                "Organization '{}' was created successfuly with id: '{}'",
                res.name, res.id
            );
        }
    };

    Ok(())
}
