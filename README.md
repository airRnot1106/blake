<div align="center">
<samp>

# blake

Tig-like terminal UI for exploring git blame history

</samp>
</div>

> **blake** = **bla**me + brea**k**
>
> Because sometimes you need to break down the blame history to find out who *really* broke it.

## Demo

![Image](https://github.com/user-attachments/assets/1d3511f4-ea05-46eb-821e-7e5f3b9c3068)

## Features

- **Recursive blame navigation**: Drill down into parent commits to trace the history of each line, with the ability to navigate back through your exploration path
- **GitHub integration**: Open commits directly in GitHub from the diff view
- **Vim-style keybindings**: Navigate efficiently with familiar keybindings
- **Customizable keymap**: Configure keybindings via TOML config file

## Requirements

- [delta](https://github.com/dandavison/delta) - Required for diff formatting

## Installation

### From source

```bash
cargo install --path .
```

### With Nix

```bash
nix run github:airRnot1106/blake -- <file>
```

## Usage

```bash
blake <file>
```

## Configuration

Configuration file is located at `~/.config/blake/config.toml`.

### Example

```toml
[keymap.blame]
"j" = "CursorDown"
"k" = "CursorUp"
"Enter" = "ShowDiff"
"," = "DrillDown"

[keymap.diff]
"j" = "ScrollDown"
"k" = "ScrollUp"
"q" = "Close"
```

## License

MIT
