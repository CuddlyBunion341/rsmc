#!/bin/bash

SRC_DIR="src"
DOC_CLIENT_DIR="docs/client"
DOC_SERVER_DIR="docs/server"
PLUGIN_DIR="plugin"

mkdir -p "$DOC_CLIENT_DIR"
mkdir -p "$DOC_SERVER_DIR"

generate_plugin_docs() {
  local dir_path=$1
  local dir_name=$(basename "$dir_path")
  local doc_type=$2

  SUBDIR_FILES=$(find "$dir_path" -type f -name '*.rs')

  if [[ $dir_path == *"client"* ]]; then
    PRELUDE_FILE="$SRC_DIR/client/prelude.rs"
    NETWORKING_FILE="$SRC_DIR/client/networking/systems.rs"
    OUTPUT_FILE="$DOC_CLIENT_DIR/${dir_name}.md"
  else
    PRELUDE_FILE="$SRC_DIR/server/prelude.rs"
    NETWORKING_FILE="$SRC_DIR/server/networking/systems.rs"
    OUTPUT_FILE="$DOC_SERVER_DIR/${dir_name}.md"
  fi

  {
    echo "file: $OUTPUT_FILE"
    echo
    echo "# Plugin Name: $dir_name"
    echo "Short plugin description for $dir_name."
    echo
    echo "## Context"
    echo "- Includes files from the project's ${dir_name} ${doc_type} plugin directory."
    echo "- Incorporates \`prelude.rs\` and networking systems specific to ${doc_type}."
    echo
    echo "## Collected Source Files"
    
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

for dir in "$SRC_DIR/client"/*; do
  if [[ -d $dir && $(find "$dir" -type f -name '*.rs') ]]; then
    generate_plugin_docs "$dir" "client"
  fi
done

for dir in "$SRC_DIR/server"/*; do
  if [[ -d $dir && $(find "$dir" -type f -name '*.rs') ]]; then
    generate_plugin_docs "$dir" "server"
  fi
done
