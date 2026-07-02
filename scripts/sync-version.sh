#!/bin/bash
set -e

VERSION=$(node -e "console.log(require('./src-tauri/tauri.conf.json').version)")
echo "Sync version: $VERSION"

sed -i '' "s/^version = .*/version = \"$VERSION\"/" src-tauri/Cargo.toml

node -e "
const p = require('./package.json');
p.version = '$VERSION';
require('fs').writeFileSync('./package.json', JSON.stringify(p, null, 2) + '\n');
"

echo "✅ $VERSION synced across tauri.conf.json, Cargo.toml, package.json"
