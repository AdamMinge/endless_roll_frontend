.PHONY: help setup install dev web desktop build build-web build-desktop clean check format lint 

help:
	@echo "Endless Roll Frontend - Available Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup         - Install all dependencies"
	@echo "  make install       - Install npm and rust dependencies"
	@echo ""
	@echo "Development:"
	@echo "  make dev           - Start web dev server (default)"
	@echo "  make web           - Start web dev server"
	@echo "  make desktop       - Start desktop dev server"
	@echo ""
	@echo "Building:"
	@echo "  make build         - Build for production (web)"
	@echo "  make build-web     - Build web version"
	@echo "  make build-desktop - Build desktop version"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make check         - Check for compilation errors"
	@echo "  make format        - Format code"
	@echo "  make lint          - Check code quality"
	@echo ""

setup: install
	@echo "✅ Setup complete!"
	@echo "Run 'make dev' to start development"

install:
	@echo "📦 Installing dependencies..."
	npm install
	@echo "✅ Dependencies installed"

dev: web

web:
	@echo "🚀 Starting web dev server..."
	npm run dev

desktop:
	@echo "🚀 Starting desktop dev server..."
	cargo tauri dev

build: build-web

build-web:
	@echo "🔨 Building web version..."
	npm run build
	@echo "✅ Build complete: dist/"

build-desktop:
	@echo "🔨 Building desktop version..."
	cargo tauri build
	@echo "✅ Build complete: src-tauri/target/release/"

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf dist node_modules target
	@echo "✅ Clean complete"

check:
	@echo "🔍 Checking for errors..."
	cargo check

format:
	@echo "📝 Formatting code..."
	cargo fmt

lint:
	@echo "🔍 Running linter..."
	cargo clippy -- -D warnings

.DEFAULT_GOAL := help
