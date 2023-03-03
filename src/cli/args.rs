use super::routers::smartbox;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Sub {
    /// Beeline SmartBox related functionality
    Smartbox {
        #[clap(subcommand)]
        sub: smartbox::Sub,
    },
}

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// Router control panel base url
    #[clap(long, short)]
    pub panel_base_url: Option<String>,

    #[clap(subcommand)]
    pub sub: Sub,
}
