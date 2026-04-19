# Windsurf Setup Guide

## Step 1: Install DSAEngine

### Option A: Using Cargo (requires Rust)
```bash
cargo install dsaengine
```

### Option B: Download Pre-built Binary
1. Go to [GitHub Releases](https://github.com/yourusername/dsaengine/releases)
2. Download `dsaengine.exe` (Windows) or `dsaengine` (macOS/Linux)
3. Move to a directory in your PATH, or note the full path

### Option C: Build from Source
```bash
git clone https://github.com/yourusername/dsaengine
cd dsaengine
cargo build --release
# Binary: target/release/dsaengine.exe (Windows) or target/release/dsaengine (macOS/Linux)
```

---

## Step 2: Configure Windsurf MCP Settings

### Create MCP Config File

Edit or create `~/.windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "dsaengine": {
      "command": "dsaengine",
      "args": ["--mcp"],
      "description": "DSAEngine — optimal algorithm advisor for Windsurf"
    }
  }
}
```

**File locations:**
- **Windows**: `%USERPROFILE%\.windsurf\mcp_config.json`
- **macOS**: `~/.windsurf/mcp_config.json`
- **Linux**: `~/.windsurf/mcp_config.json`

### Via GUI (If Available)
1. Open **Windsurf Settings**
2. Look for **MCP Servers** or **Model Context Protocol**
3. Add new server:
   - **Name**: `dsaengine`
   - **Command**: `dsaengine`
   - **Args**: `--mcp`

---

## Step 3: Restart Windsurf

1. Close Windsurf completely
2. Reopen Windsurf
3. Check the status bar — you should see DSAEngine connected

---

## Step 4: Start Coding

Ask Windsurf to write code involving algorithms:
- "Create a function to find connected components in a graph"
- "Implement merge sort from scratch"
- "Build a LRU cache"

Windsurf will automatically consult DSAEngine for the **optimal algorithm**.

---

## Verification

To verify DSAEngine is working:

1. Open **Command Palette** (⌘+⇧+P / Ctrl+Shift+P)
2. Type `MCP Server Status` or `View MCP Servers`
3. You should see `dsaengine` listed and **Connected**

---

## File Locations

If you need to manually modify the config:

**Windows:**
```cmd
C:\Users\YourUsername\.windsurf\mcp_config.json
```

**macOS/Linux:**
```bash
~/.windsurf/mcp_config.json
```

---

## Troubleshooting

### "dsaengine: command not found"
- Ensure the binary is in your PATH
- Or use the full path: `/usr/local/bin/dsaengine` or `C:\path\to\dsaengine.exe`

### "Failed to initialize MCP server"
- Test manually:
  ```bash
  dsaengine --mcp
  # Should print: [dsaengine-mcp] stdio server ready
  ```
- Check that the command path is correct in `mcp_config.json`

### "No algorithm recommendations"
- Verify `.windsurfrules` exists in your project root
- Restart Windsurf completely
- Check Windsurf's MCP server logs

---

## Advanced: Custom Port

If you want to run DSAEngine on a network port instead of stdio:

```json
{
  "mcpServers": {
    "dsaengine": {
      "command": "dsaengine",
      "args": [],
      "description": "DSAEngine HTTP server (default: localhost:8000)"
    }
  }
}
```

Then DSAEngine runs in HTTP mode (see `dsaengine --help`).

---

**You're all set!** Windsurf now has DSAEngine integration. 🚀
