use clap::Subcommand;

pub mod delete;
pub mod edit;
pub mod list;
pub mod show;

#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    #[command(name = "list")]
    /// List all configurations
    List,
    #[command(name = "show")]
    /// Prints out a single configuration
    Show {
        #[arg(long = "name")]
        name: Option<String>,
        #[arg(long = "index")]
        index: Option<usize>,
    },
    #[command(name = "delete")]
    /// Deletes a configuration
    Delete {
        #[arg(long = "name")]
        name: Option<String>,
        #[arg(long = "index")]
        index: Option<usize>,
    },
    #[command(name = "edit")]
    /// Edits a configuration using the default editor
    Edit,
}
