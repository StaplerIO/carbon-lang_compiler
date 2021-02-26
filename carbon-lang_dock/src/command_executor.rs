use carbon_logger;
use carbon_logger::logger::{log_error};

use crate::helper;

pub fn execute_command_list(list: Vec<String>) {
    if list.len() > 0 {
        match list[0].as_str() {
            "build" => {
                log_error("Command not implemented");
                return;
            }
            "run" => {
                log_error("Command not implemented");
                return;
            }
            _ => {
            }
        };
    }

    helper::display_root_command();
}