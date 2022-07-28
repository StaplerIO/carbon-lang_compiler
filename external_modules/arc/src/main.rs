use console::{Term, style};
use lazy_static::lazy_static;
use models::command_args::SubCommands;
use structopt::StructOpt;

use crate::models::command_args::CommandArgs;

mod commands;
mod models;

lazy_static! {
    static ref STDOUT: Term = Term::stdout();
}

fn main() {
    STDOUT.write_line(" -----[ Arc build system (version: 0.0.1) ]-----").unwrap();

    let args = CommandArgs::from_args();

    match args.sub_command {
        Some(SubCommands::Compile(compile_args)) => {
            commands::compile::compile_package(compile_args);
        }
        _ => {
            STDOUT.write_line(format!("{}: Not enough arguments, please check your commands", style("Error").red()).as_str()).unwrap();
        }
    }
}
