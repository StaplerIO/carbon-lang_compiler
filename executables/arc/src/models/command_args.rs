use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Arc",
    about = "The official build system for TCPL, also will be the official package manager of TCPL in the future."
)]
pub struct CommandArgs {
    #[structopt(subcommand)]
    pub sub_command: Option<SubCommands>,

    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}

#[derive(StructOpt, Debug)]
pub enum SubCommands {
    Compile(CompileCommandArgs),
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "compile",
    about = "Compile the target you specified (files or a directory) to a Carbon package."
)]
pub struct CompileCommandArgs {
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str),
        required = true,
        help = "Input a series of file or a directory contains an TCPL project (only file is supported so far)."
    )]
    // TODO: Support multiple files and directories
    pub input_path: std::path::PathBuf,

    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        required = true,
        help = "The file name where the Carbon package is stored after compilation succeeded."
    )]
    pub output_path: std::path::PathBuf,

    #[structopt(
        short = "e",
        long = "entry",
        required = false,
        default_value = "main",
        about = "The entry function name when the program starts up, whose default value is\"main\"."
    )]
    pub entry_function: String,
}
