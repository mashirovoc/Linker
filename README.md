# Linker

A CLI jump tool — register short names for paths, URLs, and files, then navigate to them instantly.

```
j add docs ~/Documents/projects/docs
j docs          # → cd ~/Documents/projects/docs

j add gh https://github.com
j gh            # → opens in browser
```

## Installation

### Windows

Download and run the installer from the **[Releases page](https://github.com/mashirovoc/Linker/releases/latest)**.

The installer will:

- Add `linker` to your system PATH
- Optionally add the `l` shell function to your PowerShell profile (PS5 and PS7)

### Linux / macOS / WSL

```sh
curl -fsSL https://raw.githubusercontent.com/mashirovoc/Linker/main/installer/install.sh | bash
```

Then reload your shell:

```sh
source ~/.bashrc   # bash
source ~/.zshrc    # zsh
```

### Build from source

```sh
cargo build --release
# Add the binary to your PATH, then set up the shell function:
linker init --shell bash        # print bash snippet → append to ~/.bashrc
linker init --shell powershell  # print PowerShell snippet → append to $PROFILE
```

## Usage

All commands are invoked via the `j` shell wrapper (or `linker` directly).

### Add an entry

```sh
j add <name> <target>
```

`<target>` can be:

- A directory path — `j <name>` will `cd` into it
- A URL (`http://`, `https://`, `ftp://`) — opens in the default browser
- A file path — opens with the default application
- An executable (`.exe`, `.bat`, `.cmd` on Windows) — launches it

### Jump to an entry

```sh
j <name>
```

Matching is tried in priority order:

1. **Exact** match
2. **Prefix** match
3. **Substring** match

If multiple entries match at the same priority level, the ambiguous matches are listed and no action is taken.

### List entries

```sh
j list
```

### Edit an entry

```sh
j edit <name> <new-target>
```

### Remove an entry

```sh
j remove <name>
```

## Configuration

Config file location:

- Windows: `%AppData%\linker\config.toml`
- Linux/macOS: `~/.config/linker/config.toml`

```toml
[opener]
url  = "default"   # command to open URLs  (e.g. "chrome")
file = "default"   # command to open files (e.g. "code")
app  = "default"   # command to launch apps
```

`"default"` uses the OS default handler (`start` on Windows, `open` on macOS, `xdg-open` on Linux).

You can also override the opener per entry using `open_with` directly in `bookmarks.json` (stored alongside `config.toml`).

## License

MIT
