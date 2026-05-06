#!/bin/bash

# Endless Roll Frontend Setup Script

set -e

echo "🎲 Endless Roll Frontend Setup"
echo "================================"

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Check for Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install it from https://nodejs.org/"
    exit 1
fi

# Check for Trunk
if ! command -v trunk &> /dev/null; then
    echo "📦 Installing Trunk..."
    cargo install trunk
fi

echo "✅ Prerequisites installed"
echo ""

# Install dependencies
echo "📦 Installing project dependencies..."
npm install

echo ""
echo "✅ Setup complete!"
echo ""
echo "Available commands:"
echo "  npm run dev       - Start development server (web)"
echo "  npm run build     - Build for production (web)"
echo "  cargo tauri dev   - Start desktop development"
echo "  cargo tauri build - Build desktop app"
echo ""
echo "📝 Next steps:"
echo "  1. Update API URL in src/api/mod.rs if needed"
echo "  2. Ensure backend is running at http://localhost:8000"
echo "  3. Run: npm run dev"
echo "  4. Open browser to http://localhost:1420"
echo ""
