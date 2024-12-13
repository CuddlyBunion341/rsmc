#!/bin/bash

# Directories
SRC_DIR="src"
DOC_DIR="docs/plugins"
PLUGIN_DIR="plugin"

# Files to concatenate
PLUGIN_FILES=$(find $PLUGIN_DIR -type f -name '*.rs')
CLIENT_PRELUDE="$SRC_DIR/client/prelude.rs"
SERVER_PRELUDE="$SRC_DIR/server/prelude.rs"
CLIENT_NETWORKING="$SRC_DIR/client/networking/systems.rs"
SERVER_NETWORKING="$SRC_DIR/server/networking/systems.rs"

# Function to concatenate files
concatenate_files() {
  local dir=$1
  find "$dir" -type f -name '*.rs' | while read -r file; do
    echo "---- File: $file ----"
    cat "$file"
    echo
  done
}

# Concatenate client and server files
echo "Concatenating client files..."
CLIENT_FILES_CONTENT=$(concatenate_files "$SRC_DIR/client")
echo "Concatenating server files..."
SERVER_FILES_CONTENT=$(concatenate_files "$SRC_DIR/server")

# Output prompt
PROMPT="# Plugin Name
Short plugin description

## Dependencies
- \`Dependency\`
  Necessary for ...

\`\`\`mermaid
A mermaid diagram showcasing the various elements of the plugin.
- Use subgraphs to structure Components / Systems / Resources / Events
- Show relations between the components systems etc.
- Show data attributes of the resources / components with the corresponding visibility
\`\`\`

## Components
- \`Component Name\`: Purpose

## Resources
- \`Resource Name\`: Purpose

## Systems
- **Category of System**:
  - \`Name of system\`: Description
- **Networking** (if applicable)
"

# Create docs directory if it doesn't exist
mkdir -p "$DOC_DIR"

# Save the output
OUTPUT_FILE="$DOC_DIR/plugin_name.md"
printf "file: %s\n\n" "$OUTPUT_FILE" > "$OUTPUT_FILE"
printf "$PROMPT" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$CLIENT_FILES_CONTENT" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$SERVER_FILES_CONTENT" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$(cat "$CLIENT_PRELUDE")" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$(cat "$SERVER_PRELUDE")" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$(cat "$CLIENT_NETWORKING")" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$(cat "$SERVER_NETWORKING")" >> "$OUTPUT_FILE"
printf "\n%s\n\n" "$PLUGIN_FILES" >> "$OUTPUT_FILE"

echo "Documentation generated and saved to $OUTPUT_FILE"
