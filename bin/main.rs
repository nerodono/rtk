use clap::Parser;
use rtk::cli::args::{CliArgs, Sub};
use rtk::cli::interface::*;

fn main() {
    let cli = CliArgs::parse();
    let pan = &cli.panel_base_url;

    match cli.sub {
        Sub::Smartbox { sub: sub_smart } => sub_smart.execute(pan),
    }
}
