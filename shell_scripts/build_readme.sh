#!/bin/bash

# 1. Dynamically find the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( dirname "$SCRIPT_DIR" )"

# 2. Configuration (using absolute paths)
OUTPUT_FILE="$PROJECT_ROOT/README.md"
SOURCE_DIR="$PROJECT_ROOT/docs/README_PARTS"

# Clear or create the output file
> "$OUTPUT_FILE"

echo "🚀 Building README.md from $SOURCE_DIR..."

# Dynamic expansion: Bash automatically reads files in numeric/alphabetical order
for filepath in "$SOURCE_DIR"/[0-9][0-9]_*.md; do
    if [ -f "$filepath" ]; then
        filename=$(basename "$filepath")
        echo "  ➕ Adding: $filename"
        cat "$filepath" >> "$OUTPUT_FILE"
        # Add a clean separator between sections
        printf '\n\n---\n\n' >> "$OUTPUT_FILE"
    fi
done

# Remove trailing blank lines and the final separator
TMP_FILE="${OUTPUT_FILE}.tmp"
sed -e :a -e '/^\n*$/{$d;N;ba' -e '}' "$OUTPUT_FILE" > "$TMP_FILE"
sed -i '$ { /^---$/d }' "$TMP_FILE"
mv "$TMP_FILE" "$OUTPUT_FILE"

echo "✅ Successfully generated $OUTPUT_FILE"