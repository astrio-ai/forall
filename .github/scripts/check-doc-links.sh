#!/usr/bin/env bash
set -euo pipefail

while IFS= read -r file; do
  [ -f "$file" ] || continue
  while IFS= read -r link; do
    case "$link" in
      http:*) continue ;;
      https:*) continue ;;
      mailto:*) continue ;;
      \#*) continue ;;
    esac
    target="${link%%#*}"
    if [ -z "$target" ]; then
      continue
    fi
    dir="$(dirname "$file")"
    resolved="${dir}/${target}"
    if [ ! -e "$resolved" ]; then
      echo "::error file=$file::broken relative link: $link (resolved to $resolved)"
      exit 1
    fi
  done < <(grep -oE '\]\([^)]+\)' "$file" | sed -E 's/^\]\(|\)$//g' || true)
done < <(
  find . \
    \( -path './.git' -o -path './target' -o -path './packages/*/node_modules' \) -prune \
    -o -type f -name '*.md' -print | sort
)
