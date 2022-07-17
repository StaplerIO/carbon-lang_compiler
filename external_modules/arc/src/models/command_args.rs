use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Arc")]
pub struct CommandArgs {
	#[structopt(subcommand)]
	sub_command: Option<SubCommands>,

	#[structopt(short, long)]
    debug: bool,

	#[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[derive(StructOpt, Debug)]
pub enum SubCommands {
	Compile(CompileCommandArgs)
}


#[derive(StructOpt, Debug)]
#[structopt(name = "compile", about = "Compile from source files or source directory to a Carbon Package")]
pub struct CompileCommandArgs {
	#[structopt(short = "i", long = "input", parse(from_os_str), required = true)]
	input_paths: Vec<std::path::PathBuf>,

	#[structopt(short = "o", long = "output", parse(from_os_str), required = true)]
	output_path: std::path::PathBuf,
}
