#!/bin/bash
set -e
OLD=${1:?Usage: ./bump-version.sh <old-version> <new-version>}
NEW=${2:?Usage: ./bump-version.sh <old-version> <new-version>}

node -e "const c=require('./src-tauri/tauri.conf.json'); c.version='$NEW'; require('fs').writeFileSync('./src-tauri/tauri.conf.json', JSON.stringify(c,null,2)+'\n')"
node -e "const p=require('./package.json'); p.version='$NEW'; require('fs').writeFileSync('./package.json', JSON.stringify(p,null,2)+'\n')"
sed -i '' "s/^version = \"$OLD\"/version = \"$NEW\"/" src-tauri/Cargo.toml

echo "Bumped $OLD → $NEW. Edit CHANGELOG.md before committing."
