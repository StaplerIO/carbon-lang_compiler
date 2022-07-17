use structopt::StructOpt;

use crate::models::command_args::CommandArgs;

mod models;

fn main() {
	let args = CommandArgs::from_args();

	println!("{:?}", args);

    println!("Hello, world!");
}
