#!/bin/bash

API_KEY=$OPENAI_API_RSMC_KEY
MODEL="gpt-4o-mini"

DOCS_DIR="docs"

call_gpt_api() {
  local prompt="$1"

  response=$(curl -s -X POST https://api.openai.com/v1/completions \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $API_KEY" \
    -d '{
      "model": "'"$MODEL"'",
      "prompt": "'"$prompt"'",
      "temperature": 0.7,
      "max_tokens": 4096,  # Adjust this according to model limits
      "top_p": 1,
      "frequency_penalty": 0.2,
      "presence_penalty": 0.2
    }'
  )

  echo "$response" | jq -r '.choices[0].text'
}

for file in "$DOCS_DIR"/*/*.md; do
  if [[ -f $file ]]; then
    file_content=$(<"$file")
    file_name=$(basename "$file")

    prompt=$(cat <<PROMPT
You are provided with a markdown document corresponding to the ECS plugin \'$file_name\'.

1. Understand the document structure.
2. Review the list of source files.
3. Analyze the source code provided at the end to identify and describe (according to the template):
   - The main **Components** and their purposes.
   - The essential **Resources** with their roles.
   - Various **Systems**, categorizing them appropriately, and descriptions of their functions.
   - Networking aspects, if applicable.
4. Create a corresponding mermaid diagram

Ensure that you don\'t miss out on any details but that the documentation serves as a nice overview and is concise.
Content:
$file_content
PROMPT
)

    response_text=$(call_gpt_api "$prompt")

    output_file="${file%.*}_processed.md"
    echo "$response_text" > "$output_file"
    echo "Processed $file and saved the response to $output_file"
  fi
done

