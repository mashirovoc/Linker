# Linker

A CLI jump tool — register short names for paths, URLs, and files, then navigate to them instantly.

```
l add docs ~/Documents/projects/docs
l docs          # → cd ~/Documents/projects/docs

l add gh https://github.com
l gh            # → opens in browser
```

## Installation

### Windows

Download and run the installer:

**[Linker-installer.exe](https://distribution.mashiro3.com/Linker-installer/0.1.0/Linker-installer.exe)**

The installer will:

- Add `linker` to your system PATH
- Optionally add the `l` shell function to your PowerShell profile (PS5 and PS7)

### Linux / macOS / WSL

```sh
curl -fsSL https://raw.githubusercontent.com/mashirovoc/linker/main/installer/install.sh | bash
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

All commands are invoked via the `l` shell wrapper (or `linker` directly).

### Add an entry

```sh
l add <name> <target>
```

`<target>` can be:

- A directory path — `l <name>` will `cd` into it
- A URL (`http://`, `https://`, `ftp://`) — opens in the default browser
- A file path — opens with the default application
- An executable (`.exe`, `.bat`, `.cmd` on Windows) — launches it

### Jump to an entry

```sh
l <name>
```

Matching is tried in priority order:

1. **Exact** match
2. **Prefix** match
3. **Substring** match

If multiple entries match at the same priority level, the ambiguous matches are listed and no action is taken.

### List entries

```sh
l list
```

### Edit an entry

```sh
l edit <name> <new-target>
```

### Remove an entry

```sh
l remove <name>
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
