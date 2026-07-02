#!/bin/bash
set -e

echo "🔎 i18n-audit — scanning for untranslated strings..."

DIRS="src"
ISSUES=0

check() {
  local PATTERN="$1"
  local LABEL="$2"

  while IFS= read -r line; do
    if [ -z "$line" ]; then continue; fi
    local file=$(echo "$line" | cut -d: -f1)
    local lnum=$(echo "$line" | cut -d: -f2)
    local content=$(echo "$line" | cut -d: -f3-)
    echo "  ⚠️  $file:$lnum — $LABEL: $content"
    ISSUES=$((ISSUES + 1))
  done < <(grep -rn "$PATTERN" $DIRS --include="*.svelte" --include="*.ts" 2>/dev/null | grep -v node_modules | grep -v ".svelte-kit" | grep -v "__tests__" | grep -v "i18n-" | grep -v "test.ts" || true)
}

echo "   → Checking aria-label..."
check 'aria-label="[A-Z]' "hardcoded aria-label"

echo "   → Checking placeholder..."
check 'placeholder="[A-Z]' "hardcoded placeholder"

echo "   → Checking title attributes..."
check ' title="[A-Z]' "hardcoded title attr"

echo "   → Checking alt attributes..."
check ' alt="[A-Z]' "hardcoded alt text"

echo "   → Checking toast messages..."
check "addToast('[A-Z]" "hardcoded toast"

echo "   → Checking error messages..."
check "Error('[A-Z]" "hardcoded error"

echo "   → Checking multi-word text nodes..."
check ">[A-Z][a-z]+ [A-Za-z]+ [A-Za-z]+<" "possible hardcoded text"

echo ""
if [ "$ISSUES" -eq 0 ]; then
  echo "✅ No untranslated strings found"
else
  echo "⚠️  $ISSUES potential untranslated strings found"
fi
exit 0  # don't block build — just report warnings
