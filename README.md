# Monaco Editor in Dioxus - Proof of Concept

## Purpose
This project demonstrates integrating Monaco Editor into a Dioxus desktop application. **This is not put together to meet a standalone code editor use-case** - rather, it shows how to embed Monaco for user code input within a larger application (e.g., for storing/managing code snippets).

## Why This Exists
While Monaco integration examples exist for other frameworks, I couldn't find a clear solution for Dioxus 0.6. This repo provides a working starting point for anyone looking to embed Monaco in their Dioxus app.

## Key Features
- ✅ Monaco Editor loaded from local assets (no CDN dependency)
- ✅ Bidirectional communication between Rust and Monaco
- ✅ Custom application menu using `muda`*
- ✅ Asset serving via Warp (required due to Dioxus asset system limitations)

* _this did fall outside my central purpose here, but as my intended initial target for my use-case was desktop, I've left this in place_

## Development Notes

### The Journey
This was built by iterating with multiple AI assistants (Claude, GPT-4/5, Gemini) to work around a mix of Dioxus 0.6 documentation gaps, gaps in the consulted models about different Dioxus versions, and my own challenges in reading/understanding the docs.

Eventually, the main challenge was getting Monaco to load locally without CDN dependencies, which ultimately required a local Warp server to serve assets.

## Prerequisites
Since there is a JavaScript dependency, additional packages are required to be installed if not already pantry staples on your system.

### Linux
```bash
# Ubuntu/Debian
sudo apt-get install libjavascriptcoregtk-4.1-dev libwebkit2gtk-4.1-dev libgtk-3-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel gtk3-devel

# Arch
sudo pacman -S webkit2gtk gtk3
```

### Windows
Requires MSVC toolchain. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022).

### macOS
Should work out of the box with Xcode Command Line Tools installed.

### Running the App
```bash
# Desktop (default)
dx serve

# Specify platform explicitly
dx serve --platform desktop
```

## Known Issues
- **Ubuntu**: Works as expected
- **Windows**: Currently experiencing styling issues in the outer application UI (under investigation)

## Technical Stack
- Dioxus 0.6 (Desktop)
- Monaco Editor (local assets)
- Warp (for serving Monaco files)
- muda (custom menus)

## Contributing
This is a minimal proof-of-concept. Feel free to use it as a foundation for your own projects!