#!/bin/bash
set -e

FILE="src/lib/i18n/i18n-svelte.ts"

if ! grep -q "import en from './en/index'" "$FILE"; then
  sed -i '' "s|import { loadedFormatters, loadedLocales } from './i18n-util'|import { loadedFormatters, loadedLocales } from './i18n-util'\nimport en from './en/index'|" "$FILE"
fi

if ! grep -q "loadedLocales.en = en" "$FILE"; then
  sed -i '' "s|initI18nSvelte<Locales, Translations, TranslationFunctions, Formatters>(loadedLocales, loadedFormatters)|initI18nSvelte<Locales, Translations, TranslationFunctions, Formatters>(loadedLocales, loadedFormatters)\n\nloadedLocales.en = en as unknown as Translations\nsetLocale('en')|" "$FILE"
fi

echo "✅ i18n-svelte.ts patched (default import + setLocale('en'))"
