# zlorbrs 🛠️

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/tristonarmstrong/zlorb/1-overview)

A lightweight, systemd-managed continuous integration tool for Git-based projects. zlorbrs monitors repositories, detects changes, and triggers builds using Bun, 
keeping your projects up-to-date effortlessly.

## 📖 Overview
zlorbrs is a Rust-based system with three components: `zlorbrs-service` (the monitoring daemon), `zlorbrs-ctl` (a CLI for easy management), and `zlorbrs-lib` (shared functionality). 
It watches Git repositories for updates, performs safe fast-forward merges, and runs build commands when changes are detected or build artifacts are missing.

## 🔑 Key Features
- 🔍 **Automatic Change Detection**: Tracks repository updates using the `git2` library.
- 🔒 **Safe Git Operations**: Ensures only fast-forward merges are applied.
- 🏗️ **Build Triggering**: Executes Bun builds when changes occur or `dist/` is missing.
- ⚙️ **Systemd Integration**: Runs reliably with automatic restarts.
- 🖥️ **CLI Management**: Easily add, remove, or list repositories without service restarts.

## 🛠️ Installation
Clone the repository:
```bash
git clone <repository-url>
```

Build and install:
```bash
just build
sudo systemctl enable zlorbrs
sudo systemctl start zlorbrs
```

## ⚙️ Configuration
zlorbrs uses JSON configuration files stored in `~/.config/zlorbrs/`. The global `service-config.json` sets the monitoring interval, while per-repository `config.json` 
files define repository-specific settings.

Example `service-config.json`:
```json
{
  "sleep_time": 60
}
```

Example repository `config.json`:
```json
{
  "name": "my-repo",
  "path": "/path/to/repo",
  "branch": "main",
  "remote": "origin",
  "build_command": "bun build"
}
```

## 🖱️ Usage
Manage repositories with `zlorbrs-ctl` commands:
```bash
# Add a repository
zlorbrs-ctl add --name my-repo --path /path/to/repo --branch main --remote origin --build-command "bun build"

# List all configured repositories
zlorbrs-ctl list

# Remove a repository
zlorbrs-ctl remove my-repo
```

## 🚀 Deployment
The `justfile` handles building and installing binaries to `/usr/local/bin/` and the systemd unit file to `/usr/lib/systemd/system/`. 
The service runs in the foreground with automatic recovery on failure, ensuring reliable operation.

## 🌟 Getting Started

1. Install zlorbrs as described above.
1. Configure your repositories using `zlorbrs-ctl add`.
1. Start the service with `sudo systemctl start zlorbrs`.
1. Monitor build logs via `journalctl -u zlorbrs`.

zlorbrs will automatically keep your repositories updated and built, saving you time and effort.

## 🤝 Contributing
Want to contribute? Check the System Architecture and Workspace Structure for details. Pull requests and feedback are welcome!
