use std::env;

use carbon_logger::logger;
use crate::command_executor::execute_command_list;

mod helper;
mod command_executor;

fn main() {
    logger::log_info("Welcome to Dock");

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    execute_command_list(args);
}
