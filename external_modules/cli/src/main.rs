use structopt::StructOpt;
use crate::models::cmdline_args::CommandLineArgs;

mod models;

fn main() {
    let args: CommandLineArgs = StructOpt::from_args();

    println!("{}", args.optimization_level);
    println!("{}", args.output_path.is_some());
    println!("{}", args.root_directory.is_some());

    println!("Hello, world!");
}
