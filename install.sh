#!/usr/bin/env bash
# Forall installer — downloads prebuilt binaries from GitHub Releases.
set -euo pipefail

REPO="${FORALL_INSTALL_REPO:-astrio-ai/forall}"
INSTALL_DIR="${FORALL_INSTALL_DIR:-${HOME}/.local/bin}"
BINARY_NAME="forall"

info() { printf '%s\n' "$*"; }
err() { printf 'forall install: %s\n' "$*" >&2; }

detect_target() {
  local os arch
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"
  case "$os" in
    darwin) os="macos" ;;
    linux) os="linux" ;;
    mingw*|msys*|cygwin*|windows*) os="windows" ;;
    *) err "unsupported OS: $os"; exit 1 ;;
  esac
  case "$arch" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *) err "unsupported architecture: $arch"; exit 1 ;;
  esac
  if [ "$os" = "windows" ]; then
    printf '%s\n' "${BINARY_NAME}-${os}-${arch}.exe"
  else
    printf '%s\n' "${BINARY_NAME}-${os}-${arch}"
  fi
}

latest_release_tag() {
  if command -v gh >/dev/null 2>&1; then
    gh release view --repo "$REPO" --json tagName -q .tagName 2>/dev/null && return 0
  fi
  curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' \
    | head -n1
}

main() {
  local target tag asset url tmp
  mkdir -p "$INSTALL_DIR"
  target="$(detect_target)"
  tag="$(latest_release_tag || true)"
  if [ -z "${tag:-}" ]; then
    err "no release found at https://github.com/${REPO}/releases yet."
    err "Check back after the first binary release is published."
    exit 1
  fi

  asset="${target}"
  url="https://github.com/${REPO}/releases/download/${tag}/${asset}"
  tmp="$(mktemp)"
  info "Installing Forall ${tag} (${target}) to ${INSTALL_DIR}/${BINARY_NAME}"

  if ! curl -fsSL "$url" -o "$tmp"; then
    err "failed to download ${url}"
    err "Expected release asset: ${asset}"
    exit 1
  fi

  chmod +x "$tmp"
  mv "$tmp" "${INSTALL_DIR}/${BINARY_NAME}"
  info "Installed ${INSTALL_DIR}/${BINARY_NAME}"
  if ! command -v "$BINARY_NAME" >/dev/null 2>&1; then
    info "Add to PATH: export PATH=\"${INSTALL_DIR}:\$PATH\""
  fi
}

main "$@"
