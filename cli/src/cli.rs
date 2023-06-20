#[derive(Debug, clap::Parser)]
pub struct Cli {
    /// Subcommand for managing a organization
    #[command(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Command {
    /// Create an organization
    CreateOrganization {
        /// Name of the organization. Must be unique
        name: String,
    },
}
