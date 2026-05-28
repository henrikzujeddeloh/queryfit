#!/usr/bin/env bash
set -euo pipefail
#
# download, extract and rename .fit files from Garmin

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
downloader_dir="$script_dir/downloader"
venv_dir="$downloader_dir/.venv"
python_download_script="$downloader_dir/main.py"

ensure_venv() {
    if [[ -x "$venv_dir/bin/python" ]] && "$venv_dir/bin/python" -c "import sys, garth; raise SystemExit(sys.version_info >= (3, 14))" >/dev/null 2>&1; then
        return
    fi

    local python_bin=""
    for candidate in python3.13 python3.12 python3.11 python3; do
        if command -v "$candidate" >/dev/null 2>&1 && "$candidate" -c "import sys; raise SystemExit(not ((3, 11) <= sys.version_info[:2] < (3, 14)))"; then
            python_bin="$candidate"
            break
        fi
    done

    if [[ -z "$python_bin" ]]; then
        echo "[ERROR] Python 3.11, 3.12, or 3.13 is required. Current garth releases do not import under Python 3.14."
        exit 1
    fi

    echo "[INFO] creating downloader virtualenv with $python_bin"
    rm -rf "$venv_dir"
    "$python_bin" -m venv "$venv_dir"
    "$venv_dir/bin/python" -m pip install -r "$downloader_dir/requirements.txt"
}

if [[ ! -f "$downloader_dir/config.ini" ]]; then
    echo "[ERROR] missing downloader/config.ini. Copy downloader/config.ini.example and fill it in first."
    exit 1
fi

ensure_venv

fit_file_folder=$("$venv_dir/bin/python" - "$downloader_dir/config.ini" <<'PY'
import configparser
import sys

config = configparser.ConfigParser()
config.read(sys.argv[1])
print(config['Garmin']['fit_file_folder'].strip())
PY
)

# download .fit files from Garmin
"$venv_dir/bin/python" "$python_download_script" || { echo "[ERROR] could not download .fit files"; exit 1; }

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
