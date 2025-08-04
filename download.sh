#!/usr/bin/env bash
#
# download, extract and rename .fit files from Garmin

fit_file_folder=$(grep 'fit_file_folder' ./downloader/config.ini | cut -d '=' -f2 | tr -d ' ')
python_download_script="downloader/main.py"

# download .fit files from Garmin
python3 "$python_download_script" || { echo "[ERROR] could not download .fit files"; exit 1; }

# extract .fit files from downloaded .zip
if ! [ -x "$(command -v unzip)" ]; then
    echo "[ERROR] unzip is not installed. Install it first.'"
    exit 1
fi
find "$fit_file_folder" -type f -name "*.zip" -print0 | while IFS= read -r -d '' zip_file; do
    unzip -q "$zip_file" -d "$fit_file_folder" || { echo "[ERROR] could not extract $zip_file"; }
done

# delete .zip files
find "$fit_file_folder" -name "*.zip" -delete || { echo "[ERROR] could not delete .zip files"; exit 1; }

# rename .fit files
# if ! [ -x "$(command -v fit-renamer)" ]; then
#     echo "[ERROR] fit-renamer is not installed. Clone fit-renamer repo, and use 'cargo install --path .'"
#     exit 1
# fi
# find "$fit_file_folder" -name "*.fit" ! -name "*_*" -exec fit-renamer {} \;
