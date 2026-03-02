#!/usr/bin/env bash
# Linker install script
# Usage: curl -fsSL https://raw.githubusercontent.com/mashiro3/linker/main/installer/install.sh | bash
set -euo pipefail

REPO="mashirovoc/Linker"
BIN="linker"
INSTALL_DIR="${HOME}/.local/bin"

# ── カラー出力 ──────────────────────────────────────────────
GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; NC='\033[0m'
info()  { echo -e "${GREEN}[linker]${NC} $*"; }
warn()  { echo -e "${YELLOW}[linker]${NC} $*"; }
error() { echo -e "${RED}[linker]${NC} $*" >&2; exit 1; }

# ── OS / アーキテクチャ検出 ──────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  OS_TAG="unknown-linux-musl" ;;
  Darwin) OS_TAG="apple-darwin" ;;
  *)      error "Unsupported OS: $OS" ;;
esac

case "$ARCH" in
  x86_64)        ARCH_TAG="x86_64" ;;
  aarch64|arm64) ARCH_TAG="aarch64" ;;
  *)             error "Unsupported architecture: $ARCH" ;;
esac

TARGET="${ARCH_TAG}-${OS_TAG}"

# ── 最新バージョン取得 ────────────────────────────────────────
if command -v curl &>/dev/null; then
  FETCH="curl -fsSL"
elif command -v wget &>/dev/null; then
  FETCH="wget -qO-"
else
  error "curl または wget が必要です"
fi

info "最新バージョンを確認中..."
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
VERSION=$(${FETCH} "$API_URL" | grep '"tag_name"' | sed 's/.*"v\([^"]*\)".*/\1/')
[ -n "$VERSION" ] || error "バージョンの取得に失敗しました (GitHub API rate limit の可能性があります)"

info "linker v${VERSION} (${TARGET}) をインストールします"

# ── バイナリダウンロード ─────────────────────────────────────
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BIN}-${TARGET}"
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

if command -v curl &>/dev/null; then
  curl -fsSL "$DOWNLOAD_URL" -o "$TMP"
else
  wget -qO "$TMP" "$DOWNLOAD_URL"
fi

# ── バイナリをインストール ────────────────────────────────────
mkdir -p "$INSTALL_DIR"
install -m 755 "$TMP" "${INSTALL_DIR}/${BIN}"
info "バイナリを ${INSTALL_DIR}/${BIN} にインストールしました"

# ── PATH / シェル関数を rc ファイルに追記 ─────────────────────
PATH_LINE='export PATH="${HOME}/.local/bin:${PATH}"'

setup_rc() {
  local rc="$1"
  [ -f "$rc" ] || return 0

  # PATH
  if ! grep -qF '.local/bin' "$rc"; then
    printf '\n# linker\n%s\n' "$PATH_LINE" >> "$rc"
    info "PATH を $rc に追加しました"
  fi

  # l 関数（重複チェック）
  if grep -qE '^function j\(\)|^j\(\)' "$rc"; then
    warn "'j' 関数はすでに $rc に存在します — スキップ"
    return 0
  fi

  printf '\n' >> "$rc"
  "${INSTALL_DIR}/${BIN}" init --shell bash >> "$rc"
  info "'l' 関数を $rc に追加しました"
}

setup_rc "${HOME}/.bashrc"
setup_rc "${HOME}/.bash_profile"
setup_rc "${HOME}/.zshrc"

# ── 完了 ─────────────────────────────────────────────────────
echo ""
info "インストール完了！次のコマンドで有効化してください："
echo ""
echo "    source ~/.bashrc   # bash の場合"
echo "    source ~/.zshrc    # zsh  の場合"
echo ""
