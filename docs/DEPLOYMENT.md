# Deployment & Distribution Guide

## 🎯 Distribution Strategy

You have **3 distribution channels**:

1. **Cargo Registry** (rust-lang.org/crates.io) — For Rust developers
2. **GitHub Releases** — Pre-built binaries for all platforms
3. **Package Managers** (Homebrew, etc.) — User-friendly installation

---

## Channel 1: Cargo Registry

### Prerequisites
- Cargo account on crates.io
- Published crate name reservation

### Steps

**1. Prepare Cargo.toml**
```toml
[package]
name = "dsaengine"
version = "0.1.0"
authors = ["Your Name <email@example.com>"]
license = "MIT"  # or your license
repository = "https://github.com/yourusername/dsaengine"
homepage = "https://github.com/yourusername/dsaengine"
documentation = "https://docs.rs/dsaengine"
description = "142+ DSA algorithms as MCP server for AI agents"
```

**2. Create account**
```bash
cargo login
# Follow prompts to log in to crates.io
```

**3. Publish**
```bash
cargo publish
```

**4. Users install with**
```bash
cargo install dsaengine
```

---

## Channel 2: GitHub Releases (Automated)

### Prerequisites
- GitHub repository
- GitHub Actions enabled

### How It Works

The GitHub Actions workflow (`.github/workflows/release.yml`) **automatically**:
1. Detects git tags (e.g., `v0.1.0`)
2. Compiles on Linux, macOS, Windows
3. Creates GitHub Release
4. Uploads pre-built binaries

### Steps

**1. Tag a release**
```bash
git tag v0.1.0
git push origin v0.1.0
```

**2. GitHub Actions automatically**:
- Builds `dsaengine` (Linux)
- Builds `dsaengine-macos-x86_64` + `dsaengine-macos-arm64`
- Builds `dsaengine.exe` (Windows)
- Creates GitHub Release with all binaries

**3. Users download from**
```
https://github.com/yourusername/dsaengine/releases
```

**4. Users run (example)**
```bash
# macOS ARM64 (Apple Silicon)
wget https://github.com/.../dsaengine-macos-arm64
chmod +x dsaengine-macos-arm64
./dsaengine-macos-arm64 --mcp

# Windows
Invoke-WebRequest https://github.com/.../dsaengine.exe -OutFile dsaengine.exe
.\dsaengine.exe --mcp
```

---

## Channel 3: Homebrew (Manual Setup)

### Create Homebrew Formula

**File**: `brew-dsaengine/Formula/dsaengine.rb`

```ruby
class Dsaengine < Formula
  desc "142+ DSA algorithms as MCP server for AI agents"
  homepage "https://github.com/yourusername/dsaengine"
  url "https://github.com/yourusername/dsaengine/archive/v0.1.0.tar.gz"
  sha256 "abc123..."  # Run: shasum -a 256 v0.1.0.tar.gz
  
  depends_on "rust" => :build
  
  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end
  
  test do
    system "#{bin}/dsaengine", "--help"
  end
end
```

### Steps

**1. Create tap repository**
```bash
# Create GitHub repo: brew-dsaengine
# Inside: Formula/dsaengine.rb (above)
```

**2. Users install with**
```bash
brew tap yourusername/dsaengine
brew install dsaengine
```

---

## 🚀 Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` with new features
- [ ] Run `cargo test` locally
- [ ] Run `cargo build --release` to verify build
- [ ] Commit changes: `git commit -am "Release v0.1.0"`
- [ ] Create git tag: `git tag v0.1.0`
- [ ] Push tag: `git push origin v0.1.0`
- [ ] **GitHub Actions automatically creates release** ✨
- [ ] Publish to crates.io: `cargo publish`
- [ ] Update Homebrew formula SHA256
- [ ] Post release notes on GitHub

---

## Setting Up Your Repository

### 1. Create GitHub Repo
```bash
git init dsaengine
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/yourusername/dsaengine
git push -u origin main
```

### 2. Verify GitHub Actions File
File: `.github/workflows/release.yml` should exist

### 3. Enable GitHub Actions
- Go to **Settings → Actions**
- Ensure "All actions and reusable workflows" is selected

### 4. Make First Release
```bash
git tag v0.1.0
git push origin v0.1.0
```

Watch **Actions** tab — should auto-build and release!

---

## Installation Instructions (For Your README)

Add to README:

```markdown
## Installation

### Option 1: Cargo (Recommended for Rust developers)
\`\`\`bash
cargo install dsaengine
\`\`\`

### Option 2: Homebrew (macOS/Linux)
\`\`\`bash
brew tap yourusername/dsaengine
brew install dsaengine
\`\`\`

### Option 3: Pre-built Binaries
Download from [GitHub Releases](https://github.com/yourusername/dsaengine/releases)

### Option 4: Build from Source
\`\`\`bash
git clone https://github.com/yourusername/dsaengine
cd dsaengine
cargo build --release
# Binary: target/release/dsaengine
\`\`\`
```

---

## Verifying Installation

Users can test:

```bash
# Check installation
dsaengine --help

# Test MCP mode
dsaengine --mcp
# Should print: [dsaengine-mcp] stdio server ready

# Test HTTP mode
dsaengine
# Should print: 🚀 dsaEngine Boot Sequence Initiated
# Visit: http://localhost:8000/swagger-ui/
```

---

## CI/CD Pipeline

Your automated workflow handles:

| Step | Tool | Trigger |
|------|------|---------|
| **Build** | Cargo | On release tag |
| **Test** | Cargo test | On PR / push |
| **Format** | Rustfmt | Lint |
| **Release** | GitHub Actions | On version tag |
| **Binary Signing** | Optional | Security |

---

## Post-Release Tasks

After tagging `v0.1.0`:

- [ ] GitHub Actions builds complete (~5-10 min)
- [ ] Binaries uploaded to GitHub Releases
- [ ] `cargo publish` to crates.io
- [ ] Update Homebrew tap with new SHA256
- [ ] Post announcement (Twitter, Reddit, etc.)
- [ ] Create blog post (optional)

---

## Version Management

Semantic Versioning: `MAJOR.MINOR.PATCH`

- **0.1.0** → Initial release
- **0.1.1** → Bug fix
- **0.2.0** → New features
- **1.0.0** → Stable release

---

## Rollback Procedures

If a release has bugs:

```bash
# Delete GitHub release tag
git tag -d v0.1.0
git push origin :refs/tags/v0.1.0

# Unpublish from crates.io (within 15 min)
cargo yank --vers 0.1.0

# Re-release as v0.1.1
git tag v0.1.1
git push origin v0.1.1
```

---

## Analytics & Monitoring

### Cargo Registry
- View download stats: https://crates.io/crates/dsaengine
- Trending algorithms tracked

### GitHub
- Star count
- Fork count
- Clone count

### Community
- GitHub Discussions
- Stack Overflow tags
- Reddit r/rust

---

## Support Channels

- **GitHub Issues** — Bug reports & feature requests
- **GitHub Discussions** — Questions & ideas
- **Email** — Direct support (optional)
- **Discord/Slack** — Community (optional)

---

## Success Metrics

Track:
- Total downloads / installs
- Active users
- Agent integrations working
- Community contributions
- GitHub stars

---

**You're ready to distribute DSAEngine! 🚀**
