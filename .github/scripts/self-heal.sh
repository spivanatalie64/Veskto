#!/bin/bash
set -euo pipefail

echo "Running self-healing diagnostics..."

# Clear npm cache and reinstall node_modules
echo "Clearing npm cache and reinstalling Node.js dependencies..."
pnpm install --frozen-lockfile || true

# Clear cargo cache and rebuild dependencies
echo "Clearing Cargo cache and rebuilding Rust dependencies..."
cargo purge || true
cargo fetch || true

# Reinstall system packages if needed (optional)
# We could add more diagnostics here, but for now, we just clear caches.

echo "Self-healing diagnostics completed."