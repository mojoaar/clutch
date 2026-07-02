#!/bin/bash
set -e

if [ $# -ne 2 ]; then
  echo "Usage: i18n-add-lang.sh <code> <name>"
  echo "Example: i18n-add-lang.sh es Spanish"
  echo "         i18n-add-lang.sh no Norwegian"
  exit 1
fi

CODE="$1"
NAME="$2"
LOCALE_DIR="src/lib/i18n"

# Validate: only 2-letter codes
if [[ ! "$CODE" =~ ^[a-z]{2}$ ]]; then
  echo "❌ Language code must be 2 lowercase letters (e.g. es, no, it)"
  exit 1
fi

# Check if already exists
if [ -d "$LOCALE_DIR/$CODE" ]; then
  echo "❌ Locale '$CODE' already exists at $LOCALE_DIR/$CODE/"
  exit 1
fi

echo "🌍 Bootstrapping locale: $CODE ($NAME)"

# Create directory
mkdir -p "$LOCALE_DIR/$CODE"

# Deep-copy en locale, prefix all values with [XX] marker
node --input-type=module -e "
import en from './src/lib/i18n/en/index.ts';
import * as fs from 'node:fs';

function deepCopy(obj) {
  if (typeof obj === 'string') return '[' + '$CODE' + '] ' + obj;
  if (obj === null || typeof obj !== 'object') return obj;
  const result = {};
  for (const [k, v] of Object.entries(obj)) {
    result[k] = deepCopy(v);
  }
  return result;
}

const translated = deepCopy(en);

// Write the locale file
const content = 'import type { Translation } from \"../i18n-types\";\n\nconst ' + '$CODE' + ': Translation = ' + JSON.stringify(translated, null, 2) + ';\n\nexport default ' + '$CODE' + ';\n';

fs.writeFileSync('$LOCALE_DIR/$CODE/index.ts', content);
console.log('✅ Created $LOCALE_DIR/$CODE/index.ts (' + Object.keys(translated).length + ' namespaces, all values prefixed with [$CODE])');
" 2>&1

echo ""
echo "✨ Locale '$CODE' bootstrapped. Next steps:"
echo "   1. Translate the [$CODE] prefixed strings in $LOCALE_DIR/$CODE/index.ts"
echo "   2. Run 'pnpm i18n-validate' to regenerate types and verify"
echo ""
echo "   The app will show '[XX] Save' for untranslated keys — no blank text."
