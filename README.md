# asdf-tui

A Terminal User Interface (TUI) for managing **asdf** plugins and tools without having to remember every command-line option.

`asdf-tui` provides an interactive interface for executing common `asdf` commands, making it easier to install, update, manage, and inspect your development toolchain directly from your terminal.

## Features

- 📦 Manage plugins
  - Add plugins
  - Remove plugins
  - Update plugins
  - List installed plugins
  - Browse available plugins

- 🛠 Manage tools
  - Install tool versions
  - Uninstall versions
  - List installed versions
  - Browse available versions
  - Set local or global versions
  - View current versions
  - Locate installed tools

- ⚡ Utilities
  - Execute arbitrary `asdf` commands
  - Recreate shims
  - View shim providers
  - Display environment information
  - Display the installed `asdf` version

- 🎨 Simple terminal interface powered by Ratatui.

## Why?

While `asdf` has an excellent command-line interface, many commands have optional flags and arguments that are easy to forget.

`asdf-tui` provides a discoverable interface that:

- Removes the need to memorise commands.
- Guides users through required parameters.
- Displays command output directly inside the terminal.
- Makes common workflows faster and more intuitive.

## Requirements

- Rust
- `asdf` installed and available on your `PATH`

Verify your installation:

```bash
asdf version
```

## Installation

Clone the repository:

```bash
git clone https://github.com/Valentine-Mario/ASDF-TUI.git
cd asdf-tui
```

Build the application:

```bash
cargo build --release
```

Run:

```bash
cargo run
```

Or execute the release binary:

```bash
./target/release/asdf-tui
```

## Demo


## How it works

`asdf-tui` is a thin wrapper around the `asdf` CLI.

Every action selected in the interface is translated into the corresponding `asdf` command and executed using Rust's `std::process::Command`.

For example:

| TUI Action | Executed Command |
|------------|------------------|
| Plugin Add | `asdf plugin add <plugin>` |
| Plugin List | `asdf plugin list` |
| Install Tool | `asdf install <tool> <version>` |
| Set Version | `asdf set <tool> <version>` |
| Reshim | `asdf reshim <tool> <version>` |

Because it delegates to the official `asdf` executable, all plugin behaviour remains identical to using `asdf` directly.

## Roadmap

- [ ] Searchable command palette
- [ ] Interactive parameter forms
- [ ] Plugin browser
- [ ] Version browser
- [ ] Theme support
- [ ] Command history
- [ ] Favourites
- [ ] Output viewer with scrolling
- [ ] Configuration options

## Contributing

Contributions, bug reports, and feature requests are welcome.

If you'd like to contribute:

1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request.

## License

MIT