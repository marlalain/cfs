# `cfs`

quickly save and retrieve values for shell scripts.

## building from source

you will need the [rust toolchain](https://rustup.rs) to run `cargo`.

```shell
cargo install cfs
```

## example usage

,ake sure `$HOME/.cargo/bin` is in your `PATH` variable.

```shell
cfs set foo bar
```

```shell
cfs get foo
```

output:

```
bar
```

## help

exert of the output of `cfs --help`.

```
Commands:
	s, set    : set a value
	g, get    : get a value
	l, list   : list all keys and values
	i, init   : Inits config file
	r, remove : remove a value
	c, clear  : clear your config file
```

## license

[BSD-3-Clause](LICENSE)