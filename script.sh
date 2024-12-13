#!/bin/bash

# Define the base directory
BASE_DIR="src"

# Find all mod.rs files and check if they contain "impl Plugin"
find "$BASE_DIR" -type f -name "mod.rs" | while read -r mod_file; do
    if grep -q "impl Plugin" "$mod_file"; then
        # Get the directory of the mod.rs file
        dir_path=$(dirname "$mod_file")
        echo "$dir_path"

        cat "$dir_path/mod.rs" | rg "::\w+_system"
    fi
done
