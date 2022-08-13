use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use json::JsonValue;
use seahorse::Context;

use crate::{get_config_path, invalid};
use crate::json_object::{get_json_object_or_create, set_json_object};

pub fn init_action(c: &Context) {
	let config_path = get_config_path();
	let path = Path::new(&config_path);

	if path.exists() {
		println!("config file already exists");
	} else {
		clear_action(c);
	}
}

pub fn list_action(c: &Context) {
	let conf = get_json_object_or_create(c.bool_flag("force-create"));

	for (key, value) in conf.entries() {
		println!("{}\t{}", key, value);
	}
}

pub fn clear_action(_c: &Context) {
	let mut file = File::create(get_config_path()).unwrap();
	write!(file, "{}", "{}").unwrap();
	println!("cleared config file at '{}'", get_config_path());
}

pub fn get_action(c: &Context) {
	if c.args.len() != 1 {
		invalid("command");
	}

	let conf = get_json_object_or_create(c.bool_flag("force-create"));
	let key = c.args.get(0);

	match key {
		None => invalid("key"),
		Some(key) => {
			if conf.has_key(&key) {
				println!("{}", conf[key]);
			} else {
				if c.bool_flag("ignore-null") {
					println!();
				} else {
					eprintln!("could not find key '{}'", key);
					exit(1);
				}
			}
		}
	}
}

pub fn set_action(c: &Context) {
	if c.args.len() != 2 {
		invalid("command");
	}

	let mut conf = get_json_object_or_create(c.bool_flag("force-create"));
	let key = c.args.get(0);

	match key {
		None => invalid("key"),
		Some(key) => {
			let value_str = c.args.get(1);

			match value_str {
				None => invalid("value"),
				Some(value_str) => {
					let json_value = JsonValue::from(value_str.as_str());
					let value = json_value.as_str().unwrap();

					if conf.has_key(key) {
						conf.remove(key);
					}

					conf.insert(key, value).unwrap();

					match set_json_object(conf) {
						Ok(_) => println!("updated config file"),
						Err(err) => eprintln!("{}", err),
					}
				}
			}
		}
	}
}

pub fn remove_action(c: &Context) {
	let mut conf = get_json_object_or_create(c.bool_flag("force-create"));
	let key = c.args.get(0);

	match key {
		None => invalid("key"),
		Some(key) => {
			if conf.has_key(&key) {
				conf.remove(&key);

				match set_json_object(conf) {
					Ok(_) => println!("updated config file"),
					Err(err) => eprintln!("{}", err),
				}
			} else {
				println!("key '{}' was not found", key);
			}
		}
	}
}
