# ASCII Banner for Zed

Turn selected text into ASCII art banners right in the editor.

<img width="2116" height="2128" alt="rec" src="https://github.com/user-attachments/assets/e985e535-5022-4326-a664-14fe62895ea5" />

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

The LSP binary is downloaded automatically from GitHub releases — no manual build steps.

### Dev extension (from source)

```sh
git clone https://github.com/rubjo/ascii-banner-zed.git
cd ascii-banner-zed/ascii-banner-zed
cargo build --release --target wasm32-wasip1
```

Then install as a dev extension:

1. Extensions panel (`cmd+shift+X`)
2. Click **Install Dev Extension**
3. Select the `ascii-banner-zed/` directory

The LSP binary is also downloaded from GitHub releases. To build the LSP from source instead:

```sh
cd ascii-banner-zed/ascii-banner-lsp
cargo build --release
export PATH="$PWD/target/release:$PATH"  # add to ~/.zshrc
```

Restart Zed. Open any text file, select text, and trigger code actions (`cmd+.`).

## Requirements

- [Zed](https://zed.dev/) editor
- [Rust](https://rustup.rs/) (only for building from source)

## License

MIT
