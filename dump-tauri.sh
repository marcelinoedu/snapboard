#!/bin/bash

OUTPUT="src-tauri-dump.txt"

# limpa o arquivo de saída
echo "DUMP DO DIRETÓRIO src-tauri" > "$OUTPUT"
echo "===========================" >> "$OUTPUT"
echo "" >> "$OUTPUT"

# percorre arquivos
find src-tauri \
  -type f \
  ! -path "*/target/*" \
  ! -path "*/node_modules/*" \
  ! -path "*/.git/*" \
  ! -name "*.lock" \
  ! -name "*.db" \
  ! -name "*.sqlite" \
  ! -name "*.png" \
  ! -name "*.jpg" \
  ! -name "*.jpeg" \
  ! -name "*.webp" \
  ! -name "*.ico" \
  ! -name "*.icns" \
| sort | while read -r file; do
    echo "----------------------------------------" >> "$OUTPUT"
    echo "FILE: $file" >> "$OUTPUT"
    echo "----------------------------------------" >> "$OUTPUT"
    echo "" >> "$OUTPUT"

    cat "$file" >> "$OUTPUT"

    echo "" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
done

echo "✅ Dump gerado em $OUTPUT"
