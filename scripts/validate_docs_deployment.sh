#!/usr/bin/env bash
set -euo pipefail

echo "Installing npm dependencies..."
npm install

echo "Building Docusaurus documentation..."
npm run docs:build

echo "Checking Git diff whitespace..."
git diff --check

echo "Listing files larger than 500 KiB..."
find . -type f -size +500k | sort

echo "Documentation deployment validation completed successfully."
