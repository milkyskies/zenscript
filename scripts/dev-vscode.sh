#!/bin/bash
# Build floe + VS Code extension, then launch a dev instance.
# Usage: ./scripts/dev-vscode.sh [path-to-open]
set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VSCODE_EXT="$REPO_ROOT/editors/vscode"

echo "Building floe..."
cargo install --path "$REPO_ROOT" --quiet

echo "Building VS Code extension..."
cd "$VSCODE_EXT"
npm install --silent 2>/dev/null
npm run compile --silent

TARGET="${1:-$REPO_ROOT}"
echo "Launching VS Code with Floe extension..."
code --extensionDevelopmentPath="$VSCODE_EXT" "$TARGET"
