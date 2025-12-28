Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "🚀 HyprBrowser Build Script"
Write-Host "============================`n"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust is not installed. Install it from https://www.rust-lang.org/tools/install"
    exit 1
}

Write-Host "✓ Rust found: $(rustc --version)`n"

Write-Host "📁 Creating directories..."
New-Item -ItemType Directory -Force -Path dist, assets | Out-Null

Write-Host "🔨 Building HyprBrowser (RELEASE ONLY)..."
cargo build --release

$binary = "target\release\hyprbrowser.exe"

Write-Host "📦 Copying executable to dist\..."
Copy-Item $binary -Destination "dist\hyprbrowser.exe" -Force

Write-Host "📦 Copying assets..."
if ((Test-Path "assets") -and ((Get-ChildItem -Path "assets" | Measure-Object).Count -gt 0)) {
    New-Item -ItemType Directory -Force -Path "dist\assets" | Out-Null
    Copy-Item "assets\*" -Destination "dist\assets" -Recurse -Force
}

Write-Host "`n✅ Release build complete!"
Write-Host "Run it with:"
Write-Host "  .\dist\hyprbrowser.exe"
