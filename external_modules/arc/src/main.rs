use console::{Term, style};
use lazy_static::lazy_static;
use managers::logging::{log_error, log_warn};
use models::command_args::SubCommands;
use structopt::StructOpt;

use crate::models::command_args::CommandArgs;

mod commands;
mod managers;
mod models;

lazy_static! {
    static ref STDOUT: Term = Term::stdout();
}

fn main() {
    STDOUT.write_line(format!("{}", style(" -----[ Arc build system (version: 0.0.1) ]-----").bold()).as_str()).unwrap();
    log_warn("This is an internal version of the Arc build system. It is not stable yet. It is still unavailable to build your project!");

    let args = CommandArgs::from_args();

    match args.sub_command {
        Some(SubCommands::Compile(compile_args)) => {
            commands::compile::compile_package(compile_args);
        }
        _ => {
            log_error("Not enough arguments, please check your commands.");
        }
    }
}
