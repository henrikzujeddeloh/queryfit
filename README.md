# queryfit
A CLI tool to analyze .fit files.

## Requirements
- rust
- python3

## Installation
Install `queryfit` with:
```bash
cargo install --path .
```

## Getting Started
Follow these steps to download .fit files from Garmin Connect.

1. In `downloader/` install required packages with `python3 -m pip install -r requirements.txt`.
2. In `downloader/` copy example config file (`cp config.ini.example config.ini`) and input Garmin Connect login credentials and directory to download .fit files to.
3. Run `./download.sh` to download latest .fit files.

## Configuration
Configuration for `queryfit` is in `~/.config/queryfit/config.toml`.

## Usage
