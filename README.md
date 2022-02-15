![Terminal](https://badgen.net/badge/icon/terminal?icon=terminal&label)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

# ASCII Image Converter

This is a small cli program written in rust to easily convert images
to ascii art.

## Usage

For simply converting an image:

```bash
ica PATH
```

For more options use:

```bash
ica -h
```

To use custom ascii chars, use the `--characters` (or `-c` for short) argument.The characters should be ordered from darkest/densest to lightest.
If the background should be invisible add a space at the end.

```bash
ica PATH -c "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789<>|,.-#+!$%&/()=?*'_:; "
```

## Building from source

If you have rust/cargo installed, you can build the project with:

```bash
cargo build
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Please be aware that it might take some time for someone to respond.

## Todo

[ ] Use directional Ascii chars

[ ] Add tests

## License

[Mozilla Public License 2.0.](LICENSE)
