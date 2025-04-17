#!/bin/bash

print_usage() {
    echo "Usage: $0 <prompt> <api_key>"
    echo "Example: $0 'What is AI?' 'sk-or-v1-...'"
    exit 1
}

if [ "$#" -ne 2 ]; then
    print_usage
fi

PROMPT="$1"
API_KEY="$2"

# OpenRouter API endpoint
API_ENDPOINT="https://openrouter.ai/api/v1/chat/completions"

# Function to make API call
make_api_call() {
    local MODEL="$1"
    echo "=== Response from $MODEL ==="
    date "+%Y-%m-%d %H:%M:%S"
    echo "---"

    response=$(curl -s "$API_ENDPOINT" \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $API_KEY" \
      -H "HTTP-Referer: https://github.com/cursor-ai" \
      -H "X-Title: Cursor AI" \
      -d "{
        \"model\": \"$MODEL\",
        \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT\"}],
        \"temperature\": 0.7
      }")

    if [[ $response == *"error"* ]]; then
        echo "Error response:"
        echo "$response" | jq '.'
    else
        echo "$response" | jq -r '.choices[0].message.content'
    fi
    echo "=== End of $MODEL response ===\n"
}

# Define model configurations
MODELS=(
    "openai/gpt-3.5-turbo"
    "anthropic/claude-3-haiku"
    "google/gemini-pro"
)

# Export necessary variables and functions for subprocess use
export PROMPT
export API_KEY
export API_ENDPOINT
export -f make_api_call

# Create temporary process compose config
TMP_COMPOSE_FILE=$(mktemp)
cat > "$TMP_COMPOSE_FILE" << EOL
version: "0.5"
log_level: debug
processes:
  monitor:
    name: "Inference Monitor"
    description: "Monitors the completion of all inference processes"
    command: "echo 'Starting inference processes...' && sleep infinity"
    availability:
      restart: "no"
      exit_on_end: true
    depends_on:
EOL

# Add model processes to depends_on
for model in "${MODELS[@]}"; do
    sanitized_name=${model//\//-}
    echo "      ${sanitized_name}:" >> "$TMP_COMPOSE_FILE"
    echo "        condition: process_completed_successfully" >> "$TMP_COMPOSE_FILE"
done

# Add model processes
for model in "${MODELS[@]}"; do
    sanitized_name=${model//\//-}
    cat >> "$TMP_COMPOSE_FILE" << EOL

  ${sanitized_name}:
    name: "Inference with ${model}"
    description: "Running inference using ${model}"
    command: bash -c 'make_api_call "${model}"'
    availability:
      restart: "no"
    working_dir: "$(pwd)"
EOL
done

# Show the generated config
echo "Generated process-compose config:"
cat "$TMP_COMPOSE_FILE"
echo "---"

# Run process-compose with cleanup
# trap 'rm -f "$TMP_COMPOSE_FILE"' EXIT
echo $TMP_COMPOSE_FILE
process-compose -f "$TMP_COMPOSE_FILE" up --ordered-shutdown