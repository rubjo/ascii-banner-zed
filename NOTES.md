# Notes

## Project structure

```
ascii-banner-lsp/          # LSP server (native Rust binary)
  src/main.rs              # LSP handler + banner rendering
  fonts/*.flf              # FIGlet font files (embedded at compile time)
  Cargo.toml
ascii-banner-zed/          # Zed extension (WASM)
  src/lib.rs               # Extension trait impl — finds & launches LSP
  extension.toml           # Extension manifest
  Cargo.toml
```

## How it works

This is a two-part project:

- **`ascii-banner-lsp/`** — a Rust LSP server that registers code actions and renders text via the `figlet-rs` crate. Fonts are embedded at compile time (no runtime downloads).
- **`ascii-banner-zed/`** — a WASM-based Zed extension that launches the LSP server when a Plain Text file is opened.

Zed's extension API doesn't yet support direct text transformation, so this works around that limitation via the LSP code action protocol.

## Local development workflow

### 1. Modify the LSP server

```sh
cd ascii-banner-lsp
# edit src/main.rs
cargo build --release
```

After rebuilding, restart Zed (or close/reopen the project) so it picks up the new binary. No changes needed to the extension.

### 2. Modify the Zed extension

```sh
cd ascii-banner-zed
# edit src/lib.rs
cargo build --release --target wasm32-wasip1
```

Then reinstall the dev extension: Extensions panel → **Install Dev Extension** → select `ascii-banner-zed/` again.

### 3. Debug logging

Launch Zed from the terminal to see `[ascii-banner-lsp]` stderr output:

```sh
zed --foreground
```

## Adding or removing fonts

1. Download a `.flf` font file (e.g. from [xero/figlet-fonts](https://github.com/xero/figlet-fonts))
2. Place it in `ascii-banner-lsp/fonts/`
3. Add/remove entries in `font_list()` in `src/main.rs`
4. Rebuild the LSP server

## Creating a GitHub release

The extension auto-downloads the LSP binary from GitHub releases. Assets must follow this naming convention:

```
ascii-banner-lsp-x86_64-unknown-linux-gnu.tar.gz
ascii-banner-lsp-x86_64-apple-darwin.tar.gz
ascii-banner-lsp-aarch64-apple-darwin.tar.gz
ascii-banner-lsp-x86_64-pc-windows-msvc.tar.gz
```

### Via CI (recommended)

Push a tag and the GitHub Actions workflow `.github/workflows/release.yml` builds and packages all platforms automatically.

### Locally

```sh
./scripts/release.sh                          # current platform
./scripts/release.sh x86_64-unknown-linux-gnu  # specific target
```

Then create a GitHub release from the [releases page](https://github.com/rubjo/ascii-banner-zed/releases) and upload the tarballs.

## Publishing to the Zed extension registry

### Prerequisites

- Fork and clone [zed-industries/extensions](https://github.com/zed-industries/extensions)
- Your extension must use one of these licenses: MIT, Apache 2.0, BSD, GPLv3, etc. — add a `LICENSE` file to your repo root.

### Naming rules

- ID must not contain `zed`, `Zed`, or `extension`
- ID should describe the purpose (e.g. `ascii-banner`)
- Do not ship the LSP binary in the extension — download it at install time
- Submodule must use an HTTPS URL (not SSH)

### Steps

1. Push your extension repo to GitHub (public)

2. Add the WASM extension as a submodule to the extensions repo:

   ```sh
   cd extensions
   git submodule add https://github.com/YOUR_USER/ascii-banner-zed.git extensions/ascii-banner
   ```

3. Edit `extensions.toml` and add:

   ```toml
   [ascii-banner]
   submodule = "extensions/ascii-banner"
   path = "ascii-banner-zed"
   version = "0.1.0"
   ```

4. Run `pnpm sort-extensions` to sort both `extensions.toml` and `.gitmodules`

5. Open a PR to `zed-industries/extensions`

### Updating

1. Update the submodule to the new commit:

   ```sh
   git submodule update --remote extensions/ascii-banner
   ```

2. Bump the version in `extensions.toml` (must match `extension.toml` in your repo)
3. Open a new PR

### Automating updates

Use the [community GitHub Action](https://github.com/huacnlee/zed-extension-action) to auto-bump the extension version on new releases.

## Current limitations

- **No custom keybinding support** — actions only accessible via code actions menu (`cmd+.`)
- **Language locked** — by default only activates on Plain Text files; other languages need manual `settings.json` config
- **Font list is hardcoded** — users can't configure font selection at runtime
