use std::{env, io};
use std::process::exit;

use seahorse::App;

use crate::commands::{clear, get_value, init, list, remove_value, set_value};

mod actions;
mod commands;
mod flags;
mod json_object;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let app = App::new(env!("CARGO_PKG_NAME"))
		.description(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage(format!("{} [commands]", env!("CARGO_PKG_NAME")))
		.command(set_value())
		.command(get_value())
		.command(list())
		.command(init())
		.command(remove_value())
		.command(clear());

	app.run(args);

	Ok(())
}

fn invalid(cause: &str) {
	eprintln!("invalid {}. get help by running `conf set --help`", cause);
	exit(1);
}

fn get_config_path() -> String {
	let home_folder = env::var("HOME").unwrap();
	let path = home_folder.to_owned() + "/.cfs.json";

	return path;
}
