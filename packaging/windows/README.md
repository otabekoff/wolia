# Windows Packaging

## Requirements

- NSIS (Nullsoft Scriptable Install System) or WiX Toolset
- Visual Studio Build Tools

## Build

```powershell
cargo build --release -p wolia-write -p wolia-grid -p wolia-deck
./build-installer.ps1
```

## Output

- `wolia-setup.exe` - NSIS installer
- `wolia.msi` - MSI installer (WiX)
