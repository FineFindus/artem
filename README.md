[![artem crate](https://img.shields.io/crates/v/artem.svg)](https://crates.io/crates/artem)
![Terminal](https://badgen.net/badge/icon/terminal?icon=terminal&label)
[![Continuous Integration](https://github.com/FineFindus/artem/actions/workflows/continuous_integration.yaml/badge.svg)](https://github.com/FineFindus/artem/actions/workflows/continuous_integration.yaml)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

# Artem

Artem is a small cli program, written in rust, to easily convert images
to ascii art, named after the latin word for art. By default it tries to use truecolor, if the terminal does not support truecolor, it falls back to 16 Color ANSI. When the ascii image is written to a file, the image will not use colors.
It supports `.jpeg`, `.png`, `.gif`, `.webp` and many more.

## Examples

### Input

\_source: https://upload.wikimedia.org/wikipedia/commons/4/44/Abraham_Lincoln_head_on_shoulders_photo_portrait.jpg
![Abraham Lincoln](/examples/abraham_lincoln.jpg)

### Output

![Abraham Lincoln](/examples/abraham_lincoln_ascii.png)

## Usage

For simply converting an image:

```bash
artem path
```

The input can either be one or multiple file paths or URLs.

**NOTE**: To use URLs, the `web_image` feature has to be enabled. It is enabled by default.

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

To change the size at which the converted image is displayed, use:

```bash
#for auto sizing height
artem PATH --height
#for auto-sizing width
artem PATH --width
#for manual resizing use the --size flag
artem PATH --size 100
```

To save the the image to a file, use the `--output` flag.

```bash
artem PATH --output ascii.txt
#if the output file is an html file, the resulting ascii art will be saved as html ascii art
artem PATH --output ascii.html
# or alternatively, use an .asn file for colored ascii art
artem PATH --output ascii.ans
```

Using the `--outline` flag, the given input image will be filtered, to only contain an outline, which will then be converted. Please be aware, that this will take some additional time, as well as that it might not perfectly work on every image. For the best result, please use and image with a clear distinction between the background and the foreground.

```bash
artem PATH --outline
```

For an even better result, it might be worthwhile trying out the `--hysteresis`/`--hys` flag, potentially with characters better suited for outlines, for example.

```bash
artem PATH --outline --hysteresis --characters "|/\_.  "
```

## Installation

### All platforms (recommended)

The easiest way to install artem is using `cargo` with

```bash
cargo install artem
```

It will automatically add `artem` to your PATH variable, so it can used like shown in the [usage section](#usage).

If `cargo` is not installed, visit the [cargo book](https://doc.rust-lang.org/cargo/getting-started/installation.html) for installation instructions.

### Linux

#### Debian-based Distributions (e.g. Ubuntu)

For Debian-based Distributions, like Ubuntu, download the `.deb` file from the [release](https://github.com/FineFindus/artem/releases) page and install it with:

```bash
sudo dpkg -i artem.deb
```

The `.deb` package also contains tab completions (for bash, zsh and fish) and a man page.

#### Archlinux-based Distributions

`artem` is available as an AUR package. You can install it with your favorite aur-helper, for example with `yay`:

```bash
yay -S artem
```

This will build it from source.
Alternatively, it is also available as a precompiled binary (`artem-bin`):

```bash
yay -S artem-bin
```

#### Other Distributions

On other distributions use the binary file provided in the [release tab](https://github.com/FineFindus/artem/releases).

Alternatively, if `brew` is installed, you can also use `brew` to install it. See the [MacOS Homebrew section](#using-homebrew) for more information.

### MacOS

#### Using Homebrew

**Warning:** Currently the brew version is outdated and can be no longer recommend. This is due to brew providing only rust version `1.59.0`. This project relies on features from `1.60.0`. The progress can be tracked at [the homebrew repo](https://github.com/Homebrew/homebrew-core/pull/98823)

The recommended way to install `artem` on MacOS is using a Homebrew [tap](https://github.com/FineFindus/homebrew-tap):

```bash
brew install finefindus/tap/artem
```

The homebrew version has the added benefit of also installing the man page and tab completions for bash, zsh and fish.

#### Binary files

Alternatively binary files (for x86_64 and Arm) are provided in the [release tab](https://github.com/FineFindus/artem/releases). This way of installing is NOT recommend over using [`brew`](#using-homebrew) or [`cargo`](#all-platforms-recommended).

### Windows

To install the windows version, without using `cargo`, download either the gnu- or the mscv compiled `.zip` files from [release tab](https://github.com/FineFindus/artem/releases) and extract the `.exe`. It should be noted that you will have to add the `.exe` manually to the PATH variable.

## Shell completions

`artem` has shell completions and a man page available. When using the homebrew version, the `.deb` package, or the aur versions, they are installed automatically, whilst for using the binary files with shell completions, the completion files, which be can be found in the compressed release file, have to be copied to the correct locations.
Assuming the compressed file has been uncompressed, use following commands to copy the files to their correct location for unix-like systems:

### Shell Completions and Man page

For **bash**:

```bash
#copy the bash completion file
sudo cp completions/artem.bash  /etc/bash_completion.d/
```

For **zsh** add the file to a `$fpath` directory:

```zsh
#copy the zsh completion file
cp completions/_artem $fpath
```

For **fish** add the file to the fish completions directory:

```fish
#copy the fish completion file
cp completions/artem.fish $HOME/.config/fish/completions/
```

For Windows add `. /path/to/_artem.ps1` (including the dot) to the PowerShell [profile](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_profiles?view=powershell-7.2).

### Man Page

`artem` also provides a man page for the binary releases contained in the `doc` directory. To view it using the `-l` flag for `man` to view a local file.

```bash
#view the local man page
man -l doc/artem.1
```

## Building from source

Assuming you have rust/cargo installed, you can build the project with:

```bash
cargo build --release
```

The `--release` flag disables debugging options, increasing performance.

Visit the [rust homepage](https://www.rust-lang.org/learn/get-started) for installation instructions if rust is not installed.

### Features

This disables the default features, whilst enabling all other specified features:

```bash
cargo build --release --no-default-features --features FEATURES
```

For more information about the usage of features, please refer to the [cargo book](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options).

The following features are currently available:

- `web_image` Accept Image URLs as input (enabled by default)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Please be aware that it might take some time for someone to respond.

## Credits/Inspiration

This projects was heavily inspired by [jp2a](https://github.com/cslarsen/jp2a) as well as
the [coding train video on ascii art](https://www.youtube.com/watch?v=55iwMYv8tGI).

Also a big thanks to [ripgrep](https://github.com/BurntSushi/ripgrep/) for indirectly helping with inspiration for the build setup.

The following images are used for testing/examples:

- [Abraham Lincoln](https://upload.wikimedia.org/wikipedia/commons/4/44/Abraham_Lincoln_head_on_shoulders_photo_portrait.jpg)
- [Radio tower](https://unsplash.com/photos/hDXk9iOi9bM)
- [Moth](https://altphotos.com/photo/deaths-head-hawkmoth-3464/)

## Todo

- [x] Better average the RGB values of multiple pixel

- [x] Use the current terminal size to auto fit the image

- [x] Support ANSI terminal colors

- [x] Convert output to colored html

- [x] Use multithreading

- [x] Add tests

- [x] Add even more test

- [x] Convert multiple files at once

- [x] Automate copying of completion files from OUT_DIR to deployment/assets

- [x] Change name

- [x] Publish

### Potential Ideas

- [x] Use edge detection and directional ascii

- [ ] Implement better resizing

## License

[Mozilla Public License 2.0.](LICENSE)
