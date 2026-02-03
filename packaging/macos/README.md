# macOS Packaging

## Requirements

- Xcode Command Line Tools
- create-dmg (optional, for DMG creation)

## Build

```bash
cargo build --release -p wolia-write -p wolia-grid -p wolia-deck
./build-app.sh
```

## Output

- `Wolia Write.app`
- `Wolia Grid.app`
- `Wolia Deck.app`
- `Wolia.dmg` - Combined DMG
