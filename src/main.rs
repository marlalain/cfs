use std::{env, io};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use std::process::exit;
use json::JsonValue;
use seahorse::{App, Command, Context, Flag, FlagType};

fn invalid_command() {
	eprintln!("invalid command. get help by running `conf set --help`");
	exit(1);
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

fn get_json_object_or_create(force_create: bool) -> JsonValue {
	let path_exists = Path::new(&get_config_path()).exists();
	if force_create && !path_exists {
		let mut file = File::create(get_config_path()).unwrap();
		write!(file, "{}", "{}").unwrap();
	}

	get_json_object()
}

fn get_json_object() -> JsonValue {
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

fn set_json_object(json: JsonValue) -> io::Result<()> {
	let mut file = File::create(get_config_path())?;
	let json_string = json::stringify_pretty(json, 2);
	write!(file, "{}", json_string)?;

	Ok(())
}

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

fn ignore_null() -> Flag {
	Flag::new("ignore-null", FlagType::Bool)
		.description("ignore null values")
		.alias("i")
}

fn force_create() -> Flag {
	Flag::new("force-create", FlagType::Bool)
		.description("creates config file, if it doesn't exist")
		.alias("f")
}

fn init_action(c: &Context) {
	let config_path = get_config_path();
	let path = Path::new(&config_path);

	if path.exists() {
		println!("config file already exists");
	} else {
		clear_action(c);
	}
}

fn list_action(c: &Context) {
	let conf = get_json_object_or_create(c.bool_flag("force-create"));

	for (key, value) in conf.entries() {
		println!("{}\t{}", key, value);
	}
}

fn clear_action(_c: &Context) {
	let mut file = File::create(get_config_path()).unwrap();
	write!(file, "{}", "{}").unwrap();
	println!("cleared config file at '{}'", get_config_path());
}

fn get_action(c: &Context) {
	if c.args.len() != 1 {
		invalid_command();
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

fn set_action(c: &Context) {
	if c.args.len() != 2 {
		invalid_command();
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
						Err(err) => eprintln!("{}", err)
					}
				}
			}
		}
	}
}

fn remove_action(c: &Context) {
	let mut conf = get_json_object_or_create(c.bool_flag("force-create"));
	let key = c.args.get(0);

	match key {
		None => invalid("key"),
		Some(key) => {
			if conf.has_key(&key) {
				conf.remove(&key);

				match set_json_object(conf) {
					Ok(_) => println!("updated config file"),
					Err(err) => eprintln!("{}", err)
				}
			} else {
				println!("key '{}' was not found", key);
			}
		}
	}
}

fn init() -> Command {
	Command::new("init")
		.description("Inits config file")
		.alias("i")
		.usage(format!("{} init", env!("CARGO_PKG_NAME")))
		.action(init_action)
}

fn list() -> Command {
	Command::new("list")
		.description("list all keys and values")
		.alias("l")
		.usage(format!("{} list", env!("CARGO_PKG_NAME")))
		.action(list_action)
		.flag(force_create())
}

fn clear() -> Command {
	Command::new("clear")
		.description("clear your config file")
		.alias("c")
		.usage(format!("{} clear", env!("CARGO_PKG_NAME")))
		.action(clear_action)
}

fn remove_value() -> Command {
	Command::new("remove")
		.description("remove a value")
		.alias("r")
		.usage(format!("{} remove foo", env!("CARGO_PKG_NAME")))
		.action(remove_action)
}

fn get_value() -> Command {
	Command::new("get")
		.description("get a value")
		.alias("g")
		.usage(format!("{} get foo", env!("CARGO_PKG_NAME")))
		.action(get_action)
		.flag(ignore_null())
		.flag(force_create())
}

fn set_value() -> Command {
	Command::new("set")
		.description("set a value")
		.alias("s")
		.usage(format!("{} set foo bar", env!("CARGO_PKG_NAME")))
		.action(set_action)
		.flag(force_create())
}