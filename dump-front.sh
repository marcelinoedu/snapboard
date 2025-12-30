#!/bin/bash

OUTPUT="frontend-dump.txt"

# limpa o arquivo de saída
echo "DUMP DO FRONTEND (React / Vite)" > "$OUTPUT"
echo "===============================" >> "$OUTPUT"
echo "" >> "$OUTPUT"

# percorre arquivos relevantes do frontend
find . \
  -type f \
  \( \
    -path "./src/*" -o \
    -path "./public/*" -o \
    -name "index.html" -o \
    -name "vite.config.*" -o \
    -name "tsconfig*.json" -o \
    -name "package.json" \
  \) \
  ! -path "*/node_modules/*" \
  ! -path "*/dist/*" \
  ! -path "*/build/*" \
  ! -path "*/.git/*" \
  ! -path "*/coverage/*" \
  ! -name "*.lock" \
  ! -name "*.log" \
  ! -name "*.map" \
  ! -name "*.png" \
  ! -name "*.jpg" \
  ! -name "*.jpeg" \
  ! -name "*.webp" \
  ! -name "*.svg" \
  ! -name "*.ico" \
| sort | while read -r file; do
    echo "----------------------------------------" >> "$OUTPUT"
    echo "FILE: $file" >> "$OUTPUT"
    echo "----------------------------------------" >> "$OUTPUT"
    echo "" >> "$OUTPUT"

    cat "$file" >> "$OUTPUT"

    echo "" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
done

echo "✅ Dump do frontend gerado em $OUTPUT"
