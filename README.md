[![Rust](https://github.com/ztroop/json-to-struct/actions/workflows/rust.yml/badge.svg)](https://github.com/ztroop/json-to-struct/actions/workflows/rust.yml)

# JSON to Struct

This is a command-line tool that generates Rust structs or TypeScript interfaces from JSON data.

## Usage

To use the tool, you can run it from the command line like this:

```sh
json-to-struct <filename> <rust|typescript>
```

Where `<filename>` is the path to a JSON file that you want to generate a schema or interface for. The second argument specifies whether to print a Rust struct or TypeScript interface.

## Examples

Here is an example of how to use the tool:

```sh
json-to-struct input.json rust
```

## Dependencies

The tool is written in Rust and depends on the `serde_json` and serde libraries for JSON parsing.

## License

This tool is licensed under the MIT license. See the LICENSE file for more information.
