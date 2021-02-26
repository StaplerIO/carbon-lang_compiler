use carbon_logger;
use carbon_logger::logger::log_info;

pub fn display_root_command()
{
    log_info("Root commands:");
    log_info("help                            - Show current page");
    log_info("build <root_file> <output_path> - Build a source file to Carbon Package (Carbon compiler required)");
    log_info("run <package_path>              - Run Carbon Package at package_path");
}