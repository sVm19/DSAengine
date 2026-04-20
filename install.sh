#!/usr/bin/env bash
# DSAEngine Installation Script

set -e

# ==========================================
# 0. Output Formatting & Colors
# ==========================================
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RESET='\033[0m'

echo -e "${BLUE} Installing dsaengine...${RESET}"

# ==========================================
# 1. Validate Required Tools
# ==========================================
if ! command -v curl >/dev/null 2>&1; then
    echo -e "${RED}❌ Error: 'curl' is required but not installed.${RESET}"
    exit 1
fi

# ==========================================
# 2. OS & Architecture Detection
# ==========================================
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

map_arch() {
    case "$1" in
        x86_64|amd64) echo "amd64" ;;
        arm64|aarch64) echo "arm64" ;;
        *) echo "unknown" ;;
    esac
}

MAPPED_ARCH=$(map_arch "$ARCH")

if [ "$MAPPED_ARCH" = "unknown" ]; then
    echo -e "${RED}❌ Error: Unsupported architecture: $ARCH${RESET}"
    exit 1
fi

if [ "$OS" != "linux" ] && [ "$OS" != "darwin" ]; then
    echo -e "${RED}❌ Error: Unsupported OS: $OS. Only Linux and macOS are supported.${RESET}"
    exit 1
fi

# Map darwin to macos
if [ "$OS" = "darwin" ]; then
    OS="macos"
fi

TARGET_BIN="dsaengine-${OS}-${MAPPED_ARCH}"
echo -e "${GREEN}✔ Detected OS/Architecture: ${OS}-${MAPPED_ARCH}${RESET}"

# ==========================================
# 3. Download Logic & Fallback
# ==========================================
INSTALL_DIR="./.dsaengine"
REPO_URL="https://github.com/sVm19/DSAengine"
DOWNLOAD_URL="${REPO_URL}/releases/latest/download/${TARGET_BIN}"

mkdir -p "$INSTALL_DIR"

echo -e "${BLUE}⬇  Attempting to download prebuilt binary...${RESET}"

# Attempt to download binary, if failure (404), switch to fallback compilation
if curl -sLf "$DOWNLOAD_URL" -o "${INSTALL_DIR}/dsaengine"; then
    echo -e "${GREEN}✔ Download successful!${RESET}"
else
    echo -e "${YELLOW}⚠️ Prebuilt binary not found or download failed.${RESET}"
    echo -e "${BLUE}⚙️  Falling back to building from source...${RESET}"
    
    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${RED}❌ Error: 'cargo' is not installed. Required for source build.${RESET}"
        exit 1
    fi
    if ! command -v git >/dev/null 2>&1; then
        echo -e "${RED}❌ Error: 'git' is not installed. Required for source build.${RESET}"
        exit 1
    fi
    
    # Clone and build
    BUILD_DIR=$(mktemp -d)
    echo -e "${BLUE}📦 Cloning repository...${RESET}"
    git clone --quiet "$REPO_URL" "$BUILD_DIR"
    
    echo -e "${BLUE}🔨 Compiling release block... (this may take a few minutes)${RESET}"
    # Move into isolated dir so we don't pollute local workspace state
    cd "$BUILD_DIR"
    cargo build --release
    cd - >/dev/null
    
    cp "${BUILD_DIR}/target/release/dsaengine" "${INSTALL_DIR}/dsaengine"
    rm -rf "$BUILD_DIR"
    echo -e "${GREEN}✔ Build complete!${RESET}"
fi

# ==========================================
# 5. Permissions
# ==========================================
chmod +x "${INSTALL_DIR}/dsaengine"

# ==========================================
# 9. Helper Script ( run-dsaengine.sh )
# ==========================================
HELPER_SCRIPT="./run-dsaengine.sh"
cat > "$HELPER_SCRIPT" << 'EOF'
#!/usr/bin/env bash
./.dsaengine/dsaengine --mcp
EOF
chmod +x "$HELPER_SCRIPT"

echo -e "${GREEN}✔ Installation successful. Tool installed to ${INSTALL_DIR}${RESET}"
echo -e "${GREEN}✔ Helper script created at: ${HELPER_SCRIPT}${RESET}"
echo -e "${BLUE}▶️  Starting MCP mode...${RESET}"
echo -e ""

# ==========================================
# 6. Run MCP Mode Automatically
# ==========================================
exec "${INSTALL_DIR}/dsaengine" --mcp
