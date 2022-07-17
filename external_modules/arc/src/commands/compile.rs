use std::fs;

use console::style;
use carbon_lang_compiler::{lexer::tokenize::tokenize, parser::{pipeline::build_whole_file, decorator::decorate_token}};

use crate::{models::command_args::CompileCommandArgs, STDOUT};

pub fn compile_package(args: CompileCommandArgs) {
	// Find out whether we are going to compile a directory or files
	if args.input_path.is_dir() {
		// Fill code later

	} else {
		let file_content = fs::read_to_string(&args.input_path);
		if file_content.is_ok() {
			let tokens = tokenize(file_content.unwrap().as_str(), true);
			let decorated_tokens = decorate_token(tokens);
			let tree = build_whole_file(decorated_tokens, args.entry_function);

			// TODO: Implement function-build in compiler library
		} else {
			STDOUT.write_line(format!("{}: coulnd't open file \"{}\"", style("Error").red(), style(args.input_path.as_path().to_str().unwrap()).green()).as_str());
		}
	}
}
