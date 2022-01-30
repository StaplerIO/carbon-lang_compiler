use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "TCPL Compiler CLI", about = "The command line interface of TCPL compiler")]
pub struct CommandLineArgs {
    #[structopt(short = "d", long = "directory", parse(from_os_str))]
    pub root_directory: Option<PathBuf>,

    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output_path: Option<PathBuf>,

    #[structopt(long = "optimize", default_value = "0")]
    pub optimization_level: u8,
}
