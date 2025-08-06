# queryfit
A CLI tool to analyze .fit files.

## Requirements
- rust
- python3

## Installation
Install `queryfit` from source:
```bash
git clone https://github.com/henrikzujeddeloh/queryfit.git
cd queryfit
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
`queryfit [GLOBAL OPTIONS] <COMMAND> [SUBCOMMAND] [COMMAND OPTIONS]`

### Global Options
- `-v` - Enable verbose output
- `-h` - Show help information

### Commands
Each command has it's own set of optional subcommands and/or arguments.

#### Database
Import .fit files into database.

- `queryfit database import` 
    - Import new .fit files into database
- `queryfit database recreate` 
    - Recreate database from all .fit files
    - Use after schema update

#### Info
Get information about app and data (.fit files and database).

- `queryfit info`

#### Summary
Output a summary of metrics over a specified time frame.

- `queryfit summary 7d [OPTIONS]` 
    - Summarize statistics over the last 7 days
- `queryfit summary 30d [OPTIONS]` 
    - Summarize statistics over the last 30 days
- `queryfit summary 365d [OPTIONS]` 
    - Summarize statistics over the last 365 days
- `queryfit summary week [WEEKNUM-YEAR] [OPTIONS]` 
    - Summarize statistics over the specified week 
    - Defaults to this week
- `queryfit summary month [MONTHNUM-YEAR] [OPTIONS]` 
    - Summarize statistics over specified month
    - Defaults to this month
- `queryfit summary year [YEAR] [OPTIONS]` 
    - Summarize statistics over specified year
    - Defaults to this year

##### Options
- `--activity <TYPE>` 
    - Filter summary by activity type (`running`, `cycling`, etc.)
    - Defaults to all activity types

