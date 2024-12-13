#!/bin/bash

SRC_DIR="src"
DOC_CLIENT_DIR="docs/client"
DOC_SERVER_DIR="docs/server"

mkdir -p "$DOC_CLIENT_DIR"
mkdir -p "$DOC_SERVER_DIR"

CLIENT_README="$DOC_CLIENT_DIR/README.md"
SERVER_README="$DOC_SERVER_DIR/README.md"

initialize_readme() {
  local output_file="$1"
  local title="$2"
  {
    echo "# $title Plugins"
    echo
    echo "A collection of plugins for the $title."
    echo
  } > "$output_file"
}

add_plugin_to_readme() {
  local readme_file="$1"
  local plugin_name="$2"
  {
    echo "* [$plugin_name](./${plugin_name}.md)"
  } >> "$readme_file"
}

generate_plugin_docs() {
  local plugin_dir="$1"
  local plugin_type="$2"
  local plugin_name=$(basename "$plugin_dir")
  local doc_dir output_file prelude_file networking_file readme_file

  if [[ "$plugin_type" == "client" ]]; then
    doc_dir="$DOC_CLIENT_DIR"
    prelude_file="$SRC_DIR/client/prelude.rs"
    networking_file="$SRC_DIR/client/networking/systems.rs"
    readme_file="$CLIENT_README"
  else
    doc_dir="$DOC_SERVER_DIR"
    prelude_file="$SRC_DIR/server/prelude.rs"
    networking_file="$SRC_DIR/server/networking/systems.rs"
    readme_file="$SERVER_README"
  fi

  output_file="$doc_dir/${plugin_name}.md"
  plugin_files=$(find "$plugin_dir" -type f -name '*.rs')

  if [[ -z "$plugin_files" ]]; then
    echo "No Rust files found in $plugin_dir. Skipping..."
    return
  fi

  add_plugin_to_readme "$readme_file" "$plugin_name"

  {
    cat <<EOF
# Plugin: $plugin_name

Short description of the $plugin_name plugin.

## Dependencies
- \`Dependency Name\`: Brief explanation of why this dependency is necessary.

## Mermaid Diagram
\`\`\`mermaid

graph TD
    %% A mermaid diagram showcasing the various elements of the plugin:
    %% - Use subgraphs to structure Components / Systems / Resources / Events
    %% - Show relations between the components systems etc.
    %% - Show data attributes of the resources / components with the corresponding visibility
    %% - Make sure to include all data fields from the events, resources, components in the diagram

\`\`\`

## Components
- \`Component Name\`: Description of purpose.

## Resources
- \`Resource Name\`: Description of purpose.

## Systems
- **Category of System**:
  - \`Name of system\`: Description
- **Networking** (if applicable)

## Context
- Includes files from the project's ${dir_name} ${doc_type} plugin directory.
- Incorporates \`prelude.rs\` and networking systems specific to ${doc_type}.

## Collected Source Files
EOF

    for file in $plugin_files; do
      echo "- $(basename "$file")"
    done

    cat <<EOF

## Source Code Content

\`\`\`rs
EOF
    for file in $plugin_files; do
      echo "// ---- File: $file ----"
      cat "$file"
      echo
    done

    echo "// ---- File: $prelude_file ----"
    cat "$prelude_file"
    echo

    echo "// ---- File: $networking_file ----"
    cat "$networking_file"
    echo

    echo "\`\`\`"
  } > "$output_file"

  echo "Generated documentation for $plugin_name: $output_file"
}

initialize_readme "$CLIENT_README" "Client"
initialize_readme "$SERVER_README" "Server"

for plugin_dir in "$SRC_DIR/client"/*; do
  if [[ -d "$plugin_dir" ]]; then
    generate_plugin_docs "$plugin_dir" "client"
  fi
done

for plugin_dir in "$SRC_DIR/server"/*; do
  if [[ -d "$plugin_dir" ]]; then
    generate_plugin_docs "$plugin_dir" "server"
  fi
done
