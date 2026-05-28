# AGENTS.md

## Project Overview

`queryfit` is a personal Garmin `.fit` data collector and analysis tool. Its current purpose is to collect `.fit` files from Garmin, import them into a local database, and generate basic statistics, summaries, and queries from that database.

The project is for the maintainer's personal use at the moment. Do not assume it needs broad public-product polish, multi-user support, hosted infrastructure, or backward compatibility beyond local data/schema concerns.

## Intended Workflow

The expected workflow is:

1. Download Garmin `.fit` files into a configured local directory.
2. Import/update the `queryfit` database from those files.
3. Run all later summaries, queries, calculations, and analysis against the database, not by repeatedly reading raw `.fit` files.

The downloader tracks sync progress with `last.txt` in the configured FIT file directory. That file records the last synced day so future downloads know where to resume.

## Architecture Notes

The Rust CLI is the main `queryfit` application. It owns database import, querying, summaries, calculations, and future analysis features.

The Python code in `downloader/` is a helper for downloading Garmin files. It uses Garmin/private API behavior through third-party libraries, so treat it as inherently more brittle than the Rust CLI.

Configuration currently exists in two places:

- `downloader/config.ini` for Garmin downloader credentials and FIT file directory.
- The `queryfit` config file, currently loaded from the user's config directory, for the main CLI data path.

## Data And Privacy

Garmin credentials, OAuth tokens, raw `.fit` files, downloaded ZIP files, local databases, and config files containing local paths or credentials are private user data.

Agents must not read raw `.fit` files unless the user explicitly asks for that specific action. Prefer working from source code, schemas, command output, and non-private metadata.

Do not print, summarize, commit, or expose credentials, OAuth tokens, `.fit` contents, or private health data.

Do not edit `downloader/config.ini`, `.garth/`, local FIT directories, local databases, or generated/private data unless the user explicitly asks.

## Development Guidelines

Prefer small, pragmatic changes. Avoid large refactors unless they are clearly needed for the requested task.

When adding capabilities, keep the current direction in mind: `queryfit` should gradually become a collector and aggregator of Garmin data with expanding query, analysis, and summary features.

Rough UI/UX and small performance issues are expected. Improve them incrementally over time rather than trying to redesign the whole app in one pass.

Do not add broad compatibility layers, public API guarantees, or complex abstractions unless there is a concrete need.

## Verification

Use judgment for testing until a fuller test suite exists.

For Rust CLI changes, prefer at least:

```bash
cargo check
```

Run more specific commands when relevant, such as:

```bash
cargo test
queryfit database import
queryfit summary 7d
```

For downloader changes, prefer syntax/import checks before running real downloads:

```bash
bash -n download.sh
python -m py_compile downloader/main.py
```

Only run `./download.sh` when it is necessary and appropriate, because it contacts Garmin and modifies the local FIT file directory and `last.txt`.

## Database And Versioning

The app version appears to be used as a database/schema compatibility marker. Treat schema changes carefully.

If changing the database schema, consider whether the app version should be bumped and whether users should be told to run:

```bash
queryfit database recreate
```

This area is not fully settled yet, so avoid making assumptions beyond the existing code behavior.

## Known Constraints

The downloader depends on Garmin behavior and third-party library support, so it may break when Garmin or dependencies change.

The analysis/query layer is intentionally rudimentary for now and expected to grow over time.

The project may contain local or generated files. Do not revert unrelated changes or clean local data unless explicitly requested.
