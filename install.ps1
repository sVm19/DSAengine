# DSAEngine Installation Script (Windows Native)
# --------------------------------------------------
# Usage: irm https://raw.githubusercontent.com/sVm19/DSAengine/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "🚀 Installing DSAEngine on Windows..." -ForegroundColor Cyan

# ==========================================
# 1. OS & Architecture Detection
# ==========================================
$Arch = if ([IntPtr]::Size -eq 8) { "amd64" } else { "x86" }
$TargetBin = "dsaengine-windows-$Arch.exe"

Write-Host "✔ Detected Architecture: $Arch" -ForegroundColor Green

# ==========================================
# 2. Paths
# ==========================================
$InstallDir = Join-Path $PWD ".dsaengine"
$RepoUrl = "https://github.com/sVm19/DSAengine"
$DownloadUrl = "$RepoUrl/releases/latest/download/$TargetBin"
$BinaryPath = Join-Path $InstallDir "dsaengine.exe"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# ==========================================
# 3. Download Logic & Fallback
# ==========================================
Write-Host "⬇  Attempting to download prebuilt binary..." -ForegroundColor Blue

try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $BinaryPath -ErrorAction Stop
    Write-Host "✔ Download successful!" -ForegroundColor Green
} catch {
    Write-Host "⚠️ Prebuilt binary not found or download failed." -ForegroundColor Yellow
    Write-Host "⚙️  Falling back to building from source..." -ForegroundColor Blue

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "❌ Error: 'cargo' (Rust) is not installed. Required for source build." -ForegroundColor Red
        Write-Host "Visit https://rustup.rs/ to install Rust."
        exit 1
    }

    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        Write-Host "❌ Error: 'git' is not installed. Required for source build." -ForegroundColor Red
        exit 1
    }

    # Clone and build
    $BuildDir = Join-Path $env:TEMP "dsaengine_build_$(Get-Random)"
    Write-Host "📦 Cloning repository (main)..." -ForegroundColor Blue
    git clone --quiet -b main "$RepoUrl" "$BuildDir"

    Write-Host "🔨 Compiling release block... (this may take a few minutes)" -ForegroundColor Blue
    Push-Location $BuildDir
    try {
        cargo build --release
    } finally {
        Pop-Location
    }

    $BuiltBinary = Join-Path $BuildDir "target\release\dsaengine.exe"
    Copy-Item $BuiltBinary $BinaryPath -Force
    Remove-Item -Path $BuildDir -Recurse -Force
    Write-Host "✔ Build complete!" -ForegroundColor Green
}

# ==========================================
# 4. Helper Script ( run-dsaengine.ps1 )
# ==========================================
$HelperScript = Join-Path $PWD "run-dsaengine.ps1"
@"
& "$BinaryPath" --mcp
"@ | Out-File -FilePath $HelperScript -Encoding utf8

Write-Host "✔ Installation successful. Tool installed to $InstallDir" -ForegroundColor Green
Write-Host "✔ Helper script created at: $HelperScript" -ForegroundColor Green
Write-Host "▶️  Starting MCP mode..." -ForegroundColor Blue

# ==========================================
# 5. Run MCP Mode
# ==========================================
& "$BinaryPath" --mcp
