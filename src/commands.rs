use seahorse::Command;

use crate::actions::{
	clear_action, get_action, init_action, list_action, remove_action, set_action,
};
use crate::flags::{force_create, ignore_null};

pub fn init() -> Command {
	Command::new("init")
		.description("inits config file")
		.alias("i")
		.usage(format!("{} init", env!("CARGO_PKG_NAME")))
		.action(init_action)
}

pub fn list() -> Command {
	Command::new("list")
		.description("list all keys and values")
		.alias("l")
		.usage(format!("{} list", env!("CARGO_PKG_NAME")))
		.action(list_action)
		.flag(force_create())
}

pub fn clear() -> Command {
	Command::new("clear")
		.description("clear your config file")
		.alias("c")
		.usage(format!("{} clear", env!("CARGO_PKG_NAME")))
		.action(clear_action)
}

pub fn remove_value() -> Command {
	Command::new("remove")
		.description("remove a value")
		.alias("r")
		.usage(format!("{} remove foo", env!("CARGO_PKG_NAME")))
		.action(remove_action)
}

pub fn get_value() -> Command {
	Command::new("get")
		.description("get a value")
		.alias("g")
		.usage(format!("{} get foo", env!("CARGO_PKG_NAME")))
		.action(get_action)
		.flag(ignore_null())
		.flag(force_create())
}

pub fn set_value() -> Command {
	Command::new("set")
		.description("set a value")
		.alias("s")
		.usage(format!("{} set foo bar", env!("CARGO_PKG_NAME")))
		.action(set_action)
		.flag(force_create())
}
