# ASCII Banner for Zed

Turn selected text into ASCII art banners right in the editor.

Select any text, open **code actions** (`cmd+.`), and pick a size:

| Action | Font | Lines |
|--------|------|-------|
| Tiny caps | Cybermedium | 4 |
| Small caps | 4Max | 5 |
| Normal | Tubes | 6 |
| Large caps | Basic | 8 |
| Huge | Georgia11 | 11 |

Multi-line selections render each line as a separate stacked banner.

## Installation

### Via Zed Extensions (recommended)

1. Open the Extensions panel (`cmd+shift+X`)
2. Search for "ASCII Banner"
3. Click **Install**

The LSP binary is downloaded automatically — no manual build steps needed.

### Build from source

Requires [Rust](https://rustup.rs/).

```sh
git clone https://github.com/rubjo/ascii-banner-zed.git
cd ascii-banner-zed/ascii-banner-lsp
cargo build --release
```

Make the binary available on your PATH:

```sh
export PATH="$PWD/target/release:$PATH"
```

Add that line to `~/.zshrc` (or your shell's config) to make it permanent.

Then install as a dev extension:

1. Extensions panel (`cmd+shift+X`)
2. Click **Install Dev Extension**
3. Select the `ascii-banner-zed/` directory

Restart Zed. Open any text file, select some text, and trigger code actions (`cmd+.`).

## Requirements

- [Rust](https://rustup.rs/) (to build the LSP server)
- [Zed](https://zed.dev/) editor

## License

MIT
