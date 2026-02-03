#!/bin/bash
# Wolia Linux Installation Script

set -e

PREFIX="${PREFIX:-/usr/local}"
BINDIR="${BINDIR:-$PREFIX/bin}"
DATADIR="${DATADIR:-$PREFIX/share}"
ICONDIR="${ICONDIR:-$DATADIR/icons/hicolor}"
APPDIR="${APPDIR:-$DATADIR/applications}"

echo "Installing Wolia to $PREFIX..."

# Install binaries
install -Dm755 target/release/wolia-write "$BINDIR/wolia-write"
install -Dm755 target/release/wolia-grid "$BINDIR/wolia-grid"
install -Dm755 target/release/wolia-deck "$BINDIR/wolia-deck"

# Install icon
for size in 16 32 48 64 128 256 512; do
    install -Dm644 "packaging/linux/icons/${size}x${size}/wolia.png" \
        "$ICONDIR/${size}x${size}/apps/wolia.png" 2>/dev/null || true
done

# Install 1024x1024 icon if available
if [ -f "wolia.png" ]; then
    install -Dm644 wolia.png "$ICONDIR/1024x1024/apps/wolia.png"
    # Also use as scalable fallback
    install -Dm644 wolia.png "$DATADIR/pixmaps/wolia.png"
fi

# Install desktop entries
install -Dm644 packaging/linux/wolia-write.desktop "$APPDIR/wolia-write.desktop"
install -Dm644 packaging/linux/wolia-grid.desktop "$APPDIR/wolia-grid.desktop"
install -Dm644 packaging/linux/wolia-deck.desktop "$APPDIR/wolia-deck.desktop"

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$ICONDIR" 2>/dev/null || true
fi

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$APPDIR" 2>/dev/null || true
fi

echo "Installation complete!"
echo ""
echo "You can now run:"
echo "  wolia-write  - Word processor"
echo "  wolia-grid   - Spreadsheet"
echo "  wolia-deck   - Presentations"
