JSON to Struct

This is a command-line tool that generates JSON schemas and TypeScript interfaces from JSON data.

## Usage

To use the tool, you can run it from the command line like this:

```sh
json-to-struct <filename>
```

Where `<filename>` is the path to a JSON file that you want to generate a schema or interface for.

The tool will output two files:

- `<filename>.jsonschema`: The JSON schema for the input data.
- `<filename>.ts`: The TypeScript interface for the input data.

## Examples

Here are some examples of how to use the tool:

```sh
json-to-struct input.json
```

## Dependencies

The tool is written in Rust and depends on the `serde_json` and serde libraries for JSON parsing.

## License

This tool is licensed under the MIT license. See the LICENSE file for more information.
