# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

## v0.3.0 - 2025-08-11
### Features
 - Summary command lists total duration and average calories per workout
 - List number of sport type in time range in summary
 - Optionally list activities in summary with `-l`
 - Show progress bar for importing .fit files to database

## v0.2.0 - 2025-08-08
### Features
 - Activity start time, distance and calories is now imported from .fit files into database
 - Summarize total distance of last 7, 30 and 365 days with optional activity filter (ex. `queryfit summary 7d --activity running`)

### Fixed
 - Save individual sessions of multi-sport activities as individual activities in database

## v0.1.0 - 2025-08-06
### Features
 - Import sport type and duration from .fit files into database (`queryfit database import`)
 - Ability to recreate database (`queryfit database recreate`)
 - View basic information about app and data (`queryfit info`)
 - Warning about database and app version mismatch to recreate database
 - Download script and respective instructions to download .fit files to desired folder
