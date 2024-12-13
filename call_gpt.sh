#!/bin/bash

API_KEY=$API_KEY
MODEL="gpt-4o-mini"

DOCS_DIR="docs"

# Used for generating links inside of the documentation
get_repository_url() {
    REMOTE_URL=$(git config --get remote.origin.url)

    if [ -z "$REMOTE_URL" ]; then
        echo "No remote origin found. Ensure that this is a valid Git repository."
        return 1
    fi

    # Convert the SSH URL to HTTP if necessary
    if [[ "$REMOTE_URL" == git@* ]]; then
        # Convert git@github.com:user/repo.git to https://github.com/user/repo
        REMOTE_URL="https://${REMOTE_URL#git@}"
        REMOTE_URL="${REMOTE_URL/:/\/}"
        REMOTE_URL="${REMOTE_URL%.git}"
    elif [[ "$REMOTE_URL" == *.git ]]; then
        # Remove .git suffix if URL is already in HTTP(S) format
        REMOTE_URL="${REMOTE_URL%.git}"
    fi

    echo "${REMOTE_URL}/blob/main/"
}

REPOSITORY_BASE_URL=$(get_repository_url)

call_gpt_api() {
  local prompt="$1"

  escaped_prompt=$(echo "$prompt" | jq -sR .)

  response=$(curl -s -X POST https://api.openai.com/v1/chat/completions \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $API_KEY" \
    -d "{
      \"model\": \"$MODEL\",
      \"messages\": [
      {
        \"role\": \"user\",
        \"content\": $escaped_prompt
      }
      ],
      \"temperature\": 0.7,
      \"max_tokens\": 4096,
      \"top_p\": 1,
      \"frequency_penalty\": 0.2,
      \"presence_penalty\": 0.2
    }"
  )

  error_message=$(echo "$response" | jq -r '.error.message // empty')
  if [ -n "$error_message" ]; then
    echo "Error: $error_message"
    exit 1
  fi

  echo "$response" | jq -r '.choices[0].message.content'
}

for file in "$DOCS_DIR"/*/*.md; do
  if [[ -f $file ]]; then
    file_content=$(<"$file")
    file_name=$(basename "$file")

    prompt=$(cat <<PROMPT
You are provided with a markdown document template corresponding to the ECS plugin \'$file_name\'.

Follow the document template and generate the corresponding markdown documentation.
Make sure that you:

- Use the appropriate text formatting
- Use concise wording
- Don\'t leave out any details
- Create links to related markdown files and entities in the documenation where relevant.
  Use the following git base url for that: $REPOSITORY_BASE_URL

Content:
$file_content
PROMPT
)

    echo "Prompting gpt for $file..."

    response_text=$(call_gpt_api "$prompt")

    output_file="${file%.*}_processed.md"
    echo "$response_text" > "$output_file"
    echo "Processed $file and saved the response to $output_file"
  fi
done

