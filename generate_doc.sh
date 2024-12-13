#!/bin/bash

# Define directories and files
SRC_DIR="src"
DOC_DIR="docs/plugins"
PLUGIN_DIR="plugin"

# Make sure the output directory exists
mkdir -p "$DOC_DIR"

# Function to process each subdirectory and generate the output file
generate_plugin_docs() {
  local dir_path=$1
  local dir_name=$(basename "$dir_path")

  # Collect relevant files from the subdirectory
  SUBDIR_FILES=$(find "$dir_path" -type f -name '*.rs')

  # Collect prelude and networking system files based on the root of the directory
  if [[ $dir_path == *"client"* ]]; then
    PRELUDE_FILE="$SRC_DIR/client/prelude.rs"
    NETWORKING_FILE="$SRC_DIR/client/networking/systems.rs"
  else
    PRELUDE_FILE="$SRC_DIR/server/prelude.rs"
    NETWORKING_FILE="$SRC_DIR/server/networking/systems.rs"
  fi

  # Output filename
  OUTPUT_FILE="$DOC_DIR/${dir_name}.md"

  # Generate the output content
  {
    echo "file: $OUTPUT_FILE"
    echo
    echo "# Plugin Name: $dir_name"
    echo "Short plugin description for $dir_name."
    echo
    echo "## Context"
    echo "- The following code has been sourced from the project's $dir_name plugin directory."
    echo "- Includes files from \`prelude.rs\` and networking systems specific to client or server."
    echo
    echo "## Collected Source Files"
    
    # List each file name
    for file in $SUBDIR_FILES; do
      echo "- $(basename "$file")"
    done

    echo
    echo "## Dependencies"
    echo "- \`Dependency\`: Describe the necessity."
    echo
    echo "\`\`\`mermaid"
    echo "graph TD"
    echo "  A[Component] --> B[System]"
    echo "  C[Resource] --> D[Event]"
    echo "\`\`\`"
    echo
    echo "## Components"
    echo "- \`Component Name\`: Purpose"
    echo
    echo "## Resources"
    echo "- \`Resource Name\`: Purpose"
    echo
    echo "## Systems"
    echo "- **Category of System**:"
    echo "  - \`Name of system\`: Description"
    echo "- **Networking** (if applicable)"
    echo

    echo "## Source Code Content"

    # Concatenate each file content into the output
    for file in $SUBDIR_FILES; do
      echo "---- File: $file ----"
      cat "$file"
      echo
    done

    # Include additional files like prelude and networking
    echo "---- File: $PRELUDE_FILE ----"
    cat "$PRELUDE_FILE"
    echo
    echo "---- File: $NETWORKING_FILE ----"
    cat "$NETWORKING_FILE"
    echo
  } > "$OUTPUT_FILE"

  echo "Documentation generated and saved to $OUTPUT_FILE"
}

# Find all subdirectories in client and server that contain .rs files
for dir in "$SRC_DIR"/client/* "$SRC_DIR"/server/*; do
  if [[ -d $dir ]]; then
    if [[ $(find "$dir" -type f -name '*.rs') ]]; then
      generate_plugin_docs "$dir"
    fi
  fi
done
