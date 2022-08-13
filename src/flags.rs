use seahorse::{Flag, FlagType};

pub fn ignore_null() -> Flag {
	Flag::new("ignore-null", FlagType::Bool)
		.description("ignore null values")
		.alias("i")
}

pub fn force_create() -> Flag {
	Flag::new("force-create", FlagType::Bool)
		.description("creates config file, if it doesn't exist")
		.alias("f")
}
