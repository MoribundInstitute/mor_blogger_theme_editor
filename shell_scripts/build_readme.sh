#!/bin/bash

# 1. Dynamically find the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( dirname "$SCRIPT_DIR" )"

# 2. Configuration (using absolute paths)
OUTPUT_FILE="$PROJECT_ROOT/README.md"
SOURCE_DIR="$PROJECT_ROOT/docs/README_PARTS"

# Exact file list matching your directory
FILES=(
    "01_header.md"
    "02_problem_solution.md"
    "03_How_to_Import_Native_GTK4_Linux_Themes.md"
    "04_core_capabilities.md"
    "05_archictecture.md"
    "06_documentation.md"
    "07_contributing.md"
    "08_ecosystem_demos.md"
    "09_getting_started.md"
)

# Clear or create the output file
> "$OUTPUT_FILE"

echo "🚀 Building README.md from $SOURCE_DIR..."

for file in "${FILES[@]}"; do
    filepath="$SOURCE_DIR/$file"
    if [ -f "$filepath" ]; then
        echo "  ➕ Adding: $file"
        cat "$filepath" >> "$OUTPUT_FILE"
        # Add a clean separator between sections
        printf '\n\n---\n\n' >> "$OUTPUT_FILE"
    else
        echo "  ⚠️  Warning: $filepath not found, skipping."
    fi
done

# Remove trailing blank lines and the final separator
TMP_FILE="${OUTPUT_FILE}.tmp"
sed -e :a -e '/^\n*$/{$d;N;ba' -e '}' "$OUTPUT_FILE" > "$TMP_FILE"
sed -i '$ { /^---$/d }' "$TMP_FILE"
mv "$TMP_FILE" "$OUTPUT_FILE"

echo "✅ Successfully generated $OUTPUT_FILE"