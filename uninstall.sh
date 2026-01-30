#!/bin/sh
set -e

# TypeHero Uninstaller
# This script removes TypeHero from your system

BOLD="$(tput bold 2>/dev/null || printf '')"
GREY="$(tput setaf 0 2>/dev/null || printf '')"
UNDERLINE="$(tput smul 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
BLUE="$(tput setaf 4 2>/dev/null || printf '')"
MAGENTA="$(tput setaf 5 2>/dev/null || printf '')"
NO_COLOR="$(tput sgr0 2>/dev/null || printf '')"

info() {
  printf '%s\n' "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
  printf '%s\n' "${YELLOW}! $*${NO_COLOR}"
}

error() {
  printf '%s\n' "${RED}x $*${NO_COLOR}" >&2
}

success() {
  printf '%s\n' "${GREEN}✓${NO_COLOR} $*"
}

confirm() {
  if [ -z "${FORCE-}" ]; then
    printf "%s " "${MAGENTA}?${NO_COLOR} $* ${BOLD}[y/N]${NO_COLOR}"
    read -r yn
    case "$yn" in
      [Yy]*) return 0 ;;
      *) return 1 ;;
    esac
  fi
  return 0
}

# Detect install location
INSTALL_DIR=""
BINARY_NAME="typehero"

# Check common installation directories
for dir in "$HOME/.cargo/bin" "$HOME/.local/bin" "/usr/local/bin" "$HOME/bin"; do
  if [ -f "$dir/$BINARY_NAME" ]; then
    INSTALL_DIR="$dir"
    break
  fi
done

if [ -z "$INSTALL_DIR" ]; then
  error "TypeHero not found in common installation directories"
  info "Checked:"
  info "  - $HOME/.cargo/bin"
  info "  - $HOME/.local/bin"
  info "  - /usr/local/bin"
  info "  - $HOME/bin"
  exit 1
fi

info "Found TypeHero at: ${BOLD}$INSTALL_DIR/$BINARY_NAME${NO_COLOR}"

# Remove binary
if confirm "Remove TypeHero binary?"; then
  rm -f "$INSTALL_DIR/$BINARY_NAME"
  success "Removed $INSTALL_DIR/$BINARY_NAME"
else
  warn "Skipped binary removal"
fi

# Ask about stats
STATS_DB="$HOME/.typehero.db"
if [ -f "$STATS_DB" ]; then
  info "Statistics database found at: ${BOLD}$STATS_DB${NO_COLOR}"
  if confirm "Remove statistics database? (this will delete all your progress)"; then
    rm -f "$STATS_DB"
    rm -f "$STATS_DB-shm" 2>/dev/null || true
    rm -f "$STATS_DB-wal" 2>/dev/null || true
    success "Removed statistics database"
  else
    info "Kept statistics database"
  fi
fi

# Legacy JSON stats
LEGACY_STATS="$HOME/.typehero_stats.json"
if [ -f "$LEGACY_STATS" ]; then
  if confirm "Remove legacy stats file ($LEGACY_STATS)?"; then
    rm -f "$LEGACY_STATS"
    success "Removed legacy stats file"
  fi
fi

echo ""
success "TypeHero uninstalled"
echo ""
info "Note: You may need to manually remove any PATH modifications from:"
info "  - ~/.bashrc"
info "  - ~/.zshrc"
info "  - ~/.profile"
info "  - ~/.config/fish/config.fish"
echo ""
info "Look for lines containing: $INSTALL_DIR"
echo ""
