# Linux Packaging

## Supported Formats

- `.deb` - Debian/Ubuntu
- `.rpm` - Fedora/RHEL
- `.AppImage` - Universal
- `.flatpak` - Flatpak

## Build

```bash
cargo build --release -p wolia-write -p wolia-grid -p wolia-deck
./build-packages.sh
```

## Output

- `wolia_0.1.0_amd64.deb`
- `wolia-0.1.0-1.x86_64.rpm`
- `Wolia-0.1.0-x86_64.AppImage`
