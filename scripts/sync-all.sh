#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="/home/natalie/Projects/Veskto"
LOG_TAG="veskto-sync"

log() {
    logger -t "$LOG_TAG" "$1"
}

cd "$REPO_DIR"

log "Starting sync: upstream -> GitHub -> GitLab"

git fetch upstream main --depth=1 --force 2>&1 | logger -t "$LOG_TAG"
git push --force origin upstream/main:refs/heads/unstable 2>&1 | logger -t "$LOG_TAG"
git push --force gitlab upstream/main:refs/heads/unstable 2>&1 | logger -t "$LOG_TAG"

log "Sync complete"
