![Terminal](https://badgen.net/badge/icon/terminal?icon=terminal&label)
[![Continuous Integration](https://github.com/FineFindus/artem/actions/workflows/continuous_integration.yaml/badge.svg)](https://github.com/FineFindus/artem/actions/workflows/continuous_integration.yaml)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

# Artem

This is a small cli program written in rust to easily convert images
to ascii art, named after the latin word for art. It will take a while to generate the ascii characters, especially for larger images. By default it tries to use truecolor, if the terminal does not support truecolor, it falls back to 16 Color ANSI. When the ascii image is written to a file, the image will not use colors.

## Examples

### Input

_source: https://upload.wikimedia.org/wikipedia/commons/4/44/Abraham_Lincoln_head_on_shoulders_photo_portrait.jpg_
![Abraham Lincoln](/examples/abraham_lincoln.jpg)

### Output

![Abraham Lincoln](/examples/abraham_lincoln_ascii.png)

## Usage

For simply converting an image:

```bash
artem PATH
```

For more options use:

```bash
artem --help
```

To use custom ascii chars, use the `--characters` (or `-c` for short) argument.The characters should be ordered from darkest/densest to lightest.
If the background should be invisible, add a space at the end. Alternatively this program has already 3 predefined character sets,
accessibly by supplying the `--characters` argument to gether with the number (`0`, `1` or `2`) of the preset that should be used.
By default preset `1` is used.

```bash
artem PATH --characters "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789<>|,.-#+!$%&/()=?*'_:; "
```

To change the size at which the converted image is displayed, use::

```bash
#for auto sizing height
artem PATH --height
#for auto-sizing width
artem PATH --width
#for manual resizing use the --size flag
artem PATH --size 100
```

## Building from source

If you have rust/cargo installed, you can build the project with:

```bash
cargo build
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Please be aware that it might take some time for someone to respond.

## Credits/Inspiration

This projects was heavily inspired by [jp2a](https://github.com/cslarsen/jp2a) as well as
the [coding train video on ascii art](https://www.youtube.com/watch?v=55iwMYv8tGI).

Also a big thanks to [ripgrep](https://github.com/BurntSushi/ripgrep/) for indirectly helping with inspiration for the build setup.

## Todo

- [x] Better average the RGB values of multiple pixel

- [x] Use the current terminal size to auto fit the image

- [x] Support ANSI terminal colors

- [ ] Convert output to colored html

- [x] Use multithreading

- [x] Add tests

- [ ] Add even more test

- [x] Change name

- [ ] Publish

## License

[Mozilla Public License 2.0.](LICENSE)
