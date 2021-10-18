#!/bin/sh

set -e

cd `dirname "$BASH_SOURCE"`
wasm-pack build --release --target=bundler
cd pkg
jq '.name = "@start9labs/argon2"' package.json > package.json.tmp
jq '.files[3] = "argon2_bg.js"' package.json.tmp > package.json
npm publish --access public
