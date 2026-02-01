use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Initialize config file with default settings")]
    Init,
    #[command(about = "Show currently applied configuration")]
    Show,
    #[command(about = "Show defaults")]
    Defaults,
}
