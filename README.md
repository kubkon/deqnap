# deqnap

Deqnap is a simple utility written in Rust 1.5 that can be used to remove
any extraneous files created by QNAP's rename merging policy. That is, the
utility traverses the entire directory tree and removes any file of the form:

```
some_filename.ext_20150916T145333.259861Z
```

In other words, it matches the following regex:

```rust
let re = Regex::new(r"_\d{8}T\d{6}\.\d{6}Z$");
```

## Usage

The utility requires one positional argument, `path`, being the root of the
directory tree to clean up. Hence, it should be run from the terminal as
follows:

```
$ deqnap <path>
```

## Building and testing

To build the utility, run in the terminal:

```
$ cargo build
```

To test it, run:

```
$ cargo test
```

## License

[MIT license.](LICENSE.md)
