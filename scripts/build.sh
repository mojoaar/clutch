#!/bin/bash
set -e

echo "🔍 Running i18n validation..."
bash scripts/i18n-validate.sh

echo ""
echo "🔨 Building Clutch..."
pnpm tauri build

echo "📦 Installing to /Applications..."
cp -r src-tauri/target/release/bundle/macos/Clutch.app /Applications/

echo "✅ Done. Clutch.app installed."
echo "  App:  /Applications/Clutch.app"
echo "  DMG:  src-tauri/target/release/bundle/dmg/Clutch_*.dmg"
