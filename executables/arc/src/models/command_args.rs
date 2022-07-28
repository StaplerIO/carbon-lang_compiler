use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Arc")]
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
    about = "Compile from source files or source directory to a Carbon Package"
)]
pub struct CompileCommandArgs {
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str),
        required = true,
        help = "Input a series of file or a directory contains an TCPL project (only file is supported so far)"
    )]
    // TODO: Support multiple files and directories
    pub input_path: std::path::PathBuf,

    #[structopt(short = "o", long = "output", parse(from_os_str), required = true)]
    pub output_path: std::path::PathBuf,

    #[structopt(short = "e", long = "entry", required = false, default_value = "main")]
    pub entry_function: String,
}
