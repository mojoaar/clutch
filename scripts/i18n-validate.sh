#!/bin/bash
set -e

echo "🔍 i18n-validate — checking locale health..."

# 1. Regenerate types from locale files
echo "   → Regenerating i18n types..."
pnpm typesafe-i18n --no-watch 2>/dev/null || npx typesafe-i18n 2>/dev/null

# 2. Apply fix-i18n.sh patch
echo "   → Patching i18n-svelte.ts..."
bash scripts/fix-i18n.sh

# Formatting generated i18n files
echo "   → Formatting i18n directory..."
npx prettier --write src/lib/i18n/i18n-types.ts src/lib/i18n/i18n-util.async.ts src/lib/i18n/i18n-util.sync.ts src/lib/i18n/i18n-util.ts src/lib/i18n/i18n-svelte.ts 2>/dev/null || true

# 3. Verify i18n-svelte.ts has required patches
I18N_SVELTE="src/lib/i18n/i18n-svelte.ts"
if ! grep -q "import en from " "$I18N_SVELTE"; then
  echo "❌ FAIL: i18n-svelte.ts missing default import of en locale"
  exit 1
fi
if ! grep -q "loadedLocales.en = en" "$I18N_SVELTE"; then
  echo "❌ FAIL: i18n-svelte.ts missing loadedLocales.en assignment"
  exit 1
fi
if ! grep -q "setLocale(" "$I18N_SVELTE"; then
  echo "❌ FAIL: i18n-svelte.ts missing setLocale('en') call"
  exit 1
fi

# 4. Cross-locale deep key comparison
echo "   → Checking cross-locale key parity..."
node --input-type=module -e "

function getLeafPaths(obj, prefix = '') {
  const result = [];
  for (const [k, v] of Object.entries(obj)) {
    const path = prefix ? prefix + '.' + k : k;
    if (v !== null && typeof v === 'object') {
      result.push(...getLeafPaths(v, path));
    } else {
      result.push(path);
    }
  }
  return result.sort();
}

function getNamespaceKeys(obj) {
  return Object.keys(obj).sort();
}

const locales = ['en', 'da', 'de', 'pl', 'fr'];
const data = {};
for (const l of locales) {
  data[l] = (await import('./src/lib/i18n/' + l + '/index.ts')).default;
}

let failed = false;

// Check namespace-level keys match
const enNamespaces = getNamespaceKeys(data.en);
for (const l of ['da', 'de', 'pl', 'fr']) {
  const ns = getNamespaceKeys(data[l]);
  const missing = enNamespaces.filter(k => !ns.includes(k));
  const extra = ns.filter(k => !enNamespaces.includes(k));
  if (missing.length) {
    console.log('  ❌ ' + l + ' missing namespaces: ' + missing.join(', '));
    failed = true;
  }
  if (extra.length) {
    console.log('  ⚠️  ' + l + ' extra namespaces (not in en): ' + extra.join(', '));
  }
}

// Check deep leaf key parity
const enLeaves = new Set(getLeafPaths(data.en));
for (const l of ['da', 'de', 'pl', 'fr']) {
  const leaves = new Set(getLeafPaths(data[l]));
  const missing = [...enLeaves].filter(k => !leaves.has(k));
  const extra = [...leaves].filter(k => !enLeaves.has(k));
  if (missing.length) {
    console.log('  ❌ ' + l + ' missing ' + missing.length + ' keys:');
    missing.slice(0, 10).forEach(k => console.log('       ' + k));
    if (missing.length > 10) console.log('       ... and ' + (missing.length - 10) + ' more');
    failed = true;
  }
  if (extra.length) {
    console.log('  ⚠️  ' + l + ' has ' + extra.length + ' extra keys (not in en):');
    extra.slice(0, 5).forEach(k => console.log('       ' + k));
  }
}

if (failed) {
  process.exit(1);
}
" 2>&1

# 5. Run vitest i18n tests (catches everything including runtime issues)
echo "   → Running i18n tests..."
npx vitest run src/__tests__/i18n.test.ts 2>&1 | tail -6

echo ""
echo "✅ i18n validation passed"
