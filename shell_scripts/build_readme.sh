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

# Dynamically discover all markdown files and sort them numerically
FILES=()
while IFS= read -r filepath; do
    FILES+=("$(basename "$filepath")")
done < <(find "$SOURCE_DIR" -maxdepth 1 -name "*.md" | sort)

for file in "${FILES[@]}"; do
    filepath="$SOURCE_DIR/$file"
    if [ -f "$filepath" ]; then
        echo "  ➕ Adding: $file"
        cat "$filepath" >> "$OUTPUT_FILE"
        # Add a clean separator between sections
        echo -e "\n\n---\n\n" >> "$OUTPUT_FILE"
    else
        echo "  ⚠️  Warning: $filepath not found, skipping."
    fi
done

# Remove the trailing separator and extra blank lines from the very end
sed -i -e :a -e '/^\n*$/{$d;N;ba' -e '}' "$OUTPUT_FILE"
sed -i '$ { /^---$/d }' "$OUTPUT_FILE"

echo "✅ Successfully generated $OUTPUT_FILE"