# Cursor-Mate

Cursor-Mate is a command-line tool for managing Cursor configuration files. It resolves the "Too many free trial accounts used on this machine" issue that occurs after deleting and re-logging into Cursor accounts.

## Features

- View Telemetry IDs information
- Generate random Telemetry IDs
- Delete configuration files
- Terminate Cursor processes
- Cross-platform support


## System Support

- Windows (x64)
- macOS (Intel x64)
- macOS (Apple Silicon)
- Linux (x64)

## Installation

[Releases](https://github.com/korykim/cursormate/releases)

## Usage

cursor-mate <command>

### Available Commands

| Command | Description |
|---------|-------------|
| `ids` | Display current Telemetry IDs information |
| `random-ids` | Generate random Telemetry IDs |
| `delete` | Delete configuration file |
| `kill` | Terminate all Cursor processes |
| `help` | Show help information |

### Options

- `-h, --help`: Show help information

## Examples

Display current IDs:
```bash
cursor-mate ids
```

Generate random IDs:
```bash
cursor-mate random-ids
```

Delete configuration file:
```bash
cursor-mate delete
```

Terminate Cursor processes:
```bash
cursor-mate kill
```

## License

[MIT](LICENSE)