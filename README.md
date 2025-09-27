# Copycat

**Copy what you cat** – a simple CLI tool to copy the contents of a file directly to your system clipboard.

---

## Features

- Copies text files to the clipboard.
- Cross-platform: works on **Linux**, **macOS**, and **Windows**.
- Prebuilt binaries available for easy use.
- Supports reading from stdin, so you can use it with pipes.

---

## Download Binaries

Prebuilt binaries are available on the [GitHub Releases page](https://github.com/sudhanvarao28/copycat/releases).

Available builds:

| Platform | Archive | Download |
|----------|---------|---------|
| Linux x86_64 | `.tar.xz` | [Download](https://github.com/sudhanvarao28/copycat/releases/download/v0.0.2/copycat-linux-x86_64.tar.xz) |
| Windows x86_64 | `.zip` | [Download](https://github.com/sudhanvarao28/copycat/releases/download/v0.0.2/copycat-windows-x86_64.zip) |
| macOS x86_64 | `.tar.gz` | [Download](https://github.com/sudhanvarao28/copycat/releases/download/v0.0.2/copycat-macos-x86_64.tar.gz) |
| macOS ARM64 | `.tar.gz` | [Download](https://github.com/sudhanvarao28/copycat/releases/download/v0.0.2/copycat-macos-arm64.tar.gz) |

---

## Installation (Optional)

You can download and extract the binaries anywhere you like.

**Linux / macOS:**

```bash
# Download and extract
curl -LO https://github.com/sudhanvarao28/copycat/releases/latest/download/copycat-linux-x86_64.tar.xz
tar -xJf copycat-linux-x86_64.tar.xz
# Make binary executable
chmod +x copycat
#Optionally you will need to remove the binary from quarantine in MACOS
xattr -d com.apple.quarantine ./copycat
# Optionally move to a directory in your PATH
sudo mv copycat /usr/local/bin/
