///!This module contains utilities for dealing with different output targets.
///!These include the shell/terminal, plain text files and text files, who support colored output.
///!For example a valid `html` file need to have certain tags, which can be added with
///!methods found in `files::html`

/// Contains methods for dealing with html files.
/// These can add starting and closing tags.
pub mod html;

/// Contains methods for converting characters to targets, who support
/// Ansi formatted colors. This includes the shell/terminal as well as `.ans`/`.ansi`
/// files.
pub mod ansi;
