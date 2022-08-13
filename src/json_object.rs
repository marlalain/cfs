use std::fs::{File, read_to_string};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use json::JsonValue;

use crate::get_config_path;

pub fn get_json_object_or_create(force_create: bool) -> JsonValue {
	let path_exists = Path::new(&get_config_path()).exists();
	if force_create && !path_exists {
		let mut file = File::create(get_config_path()).unwrap();
		write!(file, "{}", "{}").unwrap();
	}

	get_json_object()
}

pub fn get_json_object() -> JsonValue {
	let path = get_config_path();

	if !Path::new(&path).exists() {
		eprintln!("config file does not exist at '{}'", &path);
		exit(1);
	}

	let json = json::parse(&*read_to_string(&path).unwrap()).unwrap();

	if !json.is_object() {
		eprintln!("config file is not a JSON file ('{}')", &path);
		exit(1);
	}

	return json;
}

pub fn set_json_object(json: JsonValue) -> io::Result<()> {
	let mut file = File::create(get_config_path())?;
	let json_string = json::stringify_pretty(json, 2);
	write!(file, "{}", json_string)?;

	Ok(())
}
