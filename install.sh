#!/bin/bash
# Dual Engine - Global Installation Script (Linux/Mac)
# Run: chmod +x install.sh && ./install.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "========================================="
echo "Dual Engine - Global Installation"
echo "========================================="
echo ""

BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

echo "Copying binaries..."
cp "$PROJECT_DIR/bin/de" "$BIN_DIR/de"
cp "$PROJECT_DIR/bin/det" "$BIN_DIR/det"
cp "$PROJECT_DIR/bin/opencode" "$BIN_DIR/opencode" 2>/dev/null || true
cp "$PROJECT_DIR/bin/claude" "$BIN_DIR/claude" 2>/dev/null || true
chmod +x "$BIN_DIR/de" "$BIN_DIR/det"

# Add to PATH
SHELL_RC="$HOME/.bashrc"
if [[ "$OSTYPE" == "darwin"* ]]; then
    SHELL_RC="$HOME/.zshrc"
fi

if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
    echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
fi

echo ""
echo "========================================="
echo "Installation complete!"
echo ""
echo "Commands:"
echo "  de    - CLI mode"
echo "  det   - TUI mode"
echo ""
echo "Please restart your terminal or run:"
echo "  source $SHELL_RC"
echo "========================================="