#!/bin/bash
set -e

if [ $# -lt 2 ]; then
  echo "Usage: i18n-add-key.sh <dot.path> <en> [da] [de] [pl] [fr]"
  echo "Example: i18n-add-key.sh skills.verified \"Verified\" \"Verificeret\" \"Verifiziert\" \"Zweryfikowano\" \"Vérifié\""
  echo ""
  echo "If only en value is given, it's copied to all locales (prefix with [XX] marker)."
  exit 1
fi

KEY_PATH="$1"
EN_VALUE="$2"
DA_VALUE="${3:-[da] $EN_VALUE}"
DE_VALUE="${4:-[de] $EN_VALUE}"
PL_VALUE="${5:-[pl] $EN_VALUE}"
FR_VALUE="${6:-[fr] $EN_VALUE}"

# Split path: "skills.verified" → namespace="skills", key="verified"
NAMESPACE="${KEY_PATH%%.*}"
KEY="${KEY_PATH#*.}"

if [ "$NAMESPACE" = "$KEY_PATH" ]; then
  echo "❌ Key path must contain a dot (namespace.key)"
  exit 1
fi

LOCALE_DIR="src/lib/i18n"
LOCALES=("en" "da" "de" "pl" "fr")
VALUES=("$EN_VALUE" "$DA_VALUE" "$DE_VALUE" "$PL_VALUE" "$FR_VALUE")

for i in "${!LOCALES[@]}"; do
  LOCALE="${LOCALES[$i]}"
  VALUE="${VALUES[$i]}"
  FILE="$LOCALE_DIR/$LOCALE/index.ts"

  # Check if key already exists
  if grep -q "^\s*$KEY:" "$FILE"; then
    echo "⚠️  $LOCALE: key '$KEY' already exists in $FILE (skipping)"
    continue
  fi

  # Find the namespace block and add the key before the closing brace
  # Strategy: find the line with "$NAMESPACE: {" and insert after it
  NAMESPACE_LINE=$(grep -n "^  $NAMESPACE: {" "$FILE" | head -1 | cut -d: -f1)
  if [ -z "$NAMESPACE_LINE" ]; then
    echo "❌ $LOCALE: namespace '$NAMESPACE' not found in $FILE"
    exit 1
  fi

  # Insert the new key after the namespace opening line
  sed -i '' "${NAMESPACE_LINE}a\\
    $KEY: '$VALUE'," "$FILE"
  
  echo "✅ $LOCALE: added $KEY_PATH = '$VALUE'"
done

echo ""
echo "✨ Keys added. Run 'pnpm i18n-validate' to regenerate types and verify."
