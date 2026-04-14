#!/usr/bin/env bash
# =============================================================================
#  LumiSync Inventory & Sales System — One-Command Setup
#  Usage:
#    ./setup.sh              → Desktop app (Tauri) — default
#    ./setup.sh desktop      → Desktop app (Tauri)
#    ./setup.sh web          → Web server mode (Node.js serve + backend)
#    ./setup.sh dev          → Development mode (hot-reload)
#    ./setup.sh dev-web      → Development web mode
# =============================================================================
set -euo pipefail

MODE="${1:-desktop}"
RESET='\033[0m'
BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'

banner() {
  echo -e "${CYAN}"
  echo "  ██╗     ██╗   ██╗███╗   ███╗██╗███████╗██╗   ██╗███╗   ██╗ ██████╗ "
  echo "  ██║     ██║   ██║████╗ ████║██║██╔════╝╚██╗ ██╔╝████╗  ██║██╔════╝ "
  echo "  ██║     ██║   ██║██╔████╔██║██║███████╗ ╚████╔╝ ██╔██╗ ██║██║      "
  echo "  ██║     ██║   ██║██║╚██╔╝██║██║╚════██║  ╚██╔╝  ██║╚██╗██║██║      "
  echo "  ███████╗╚██████╔╝██║ ╚═╝ ██║██║███████║   ██║   ██║ ╚████║╚██████╗ "
  echo "  ╚══════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═══╝ ╚═════╝ "
  echo -e "${RESET}"
  echo -e "  ${BOLD}Inventory & Sales System — Philippines${RESET}"
  echo -e "  Mode: ${YELLOW}${MODE}${RESET}"
  echo ""
}

info()    { echo -e "  ${GREEN}[✓]${RESET} $1"; }
warning() { echo -e "  ${YELLOW}[!]${RESET} $1"; }
error()   { echo -e "  ${RED}[✗]${RESET} $1"; exit 1; }
step()    { echo -e "\n  ${BOLD}${CYAN}→${RESET} ${BOLD}$1${RESET}"; }

# ── Check Dependencies ────────────────────────────────────────────────────────

check_command() {
  local cmd="$1"
  local name="${2:-$1}"
  local install_hint="${3:-}"
  if command -v "$cmd" &>/dev/null; then
    info "$name found: $(command -v "$cmd")"
    return 0
  else
    warning "$name not found"
    if [[ -n "$install_hint" ]]; then
      echo "     → $install_hint"
    fi
    return 1
  fi
}

install_node_deps() {
  step "Installing Node.js dependencies"
  if command -v pnpm &>/dev/null; then
    pnpm install
  elif command -v yarn &>/dev/null; then
    yarn install
  else
    npm install
  fi
  info "Node.js dependencies installed"
}

# ── Desktop Mode (Tauri) ──────────────────────────────────────────────────────

setup_desktop() {
  step "Checking system requirements for Desktop (Tauri) build"

  # Node.js
  check_command node "Node.js" "Install from: https://nodejs.org (LTS recommended)" || {
    # Try to install nvm + node automatically on Linux
    if [[ "$(uname)" == "Linux" ]]; then
      warning "Attempting to install Node.js via nvm…"
      curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
      export NVM_DIR="$HOME/.nvm"
      # shellcheck disable=SC1091
      [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
      nvm install --lts && nvm use --lts
    else
      error "Please install Node.js from https://nodejs.org and re-run this script"
    fi
  }

  # Rust
  check_command rustc "Rust" "Install from: https://rustup.rs" || {
    warning "Attempting to install Rust via rustup…"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
  }

  check_command cargo "Cargo (Rust)" || source "$HOME/.cargo/env"

  # Tauri CLI
  if ! command -v cargo-tauri &>/dev/null && ! npm ls @tauri-apps/cli &>/dev/null 2>&1; then
    warning "Installing @tauri-apps/cli locally…"
    npm install --save-dev @tauri-apps/cli 2>/dev/null || true
  fi

  # System dependencies (Linux)
  if [[ "$(uname)" == "Linux" ]]; then
    step "Checking Linux system libraries"
    local missing_deps=()
    local packages=(
      libwebkit2gtk-4.0-dev
      build-essential
      curl
      wget
      libssl-dev
      libgtk-3-dev
      libayatana-appindicator3-dev
      librsvg2-dev
    )
    for pkg in "${packages[@]}"; do
      if ! dpkg -s "$pkg" &>/dev/null 2>&1; then
        missing_deps+=("$pkg")
      fi
    done
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
      warning "Installing missing system libraries: ${missing_deps[*]}"
      sudo apt-get update -qq
      sudo apt-get install -y "${missing_deps[@]}"
      info "System libraries installed"
    else
      info "All system libraries present"
    fi
  fi

  install_node_deps

  step "Building desktop application (this may take a few minutes on first build)"
  npm run tauri:build

  info "Build complete!"
  echo ""
  echo -e "  ${GREEN}${BOLD}✓ Desktop application built successfully!${RESET}"
  echo ""
  echo "  📦 Installer located in: src-tauri/target/release/bundle/"
  echo ""
  echo "  To run the app directly:"
  echo "    ./src-tauri/target/release/lumisync"
  echo ""
  echo "  Default login: admin / Admin@123"
  echo ""
}

# ── Development Mode ──────────────────────────────────────────────────────────

setup_dev() {
  step "Checking development requirements"
  check_command node "Node.js" "Install from: https://nodejs.org" || error "Node.js is required"
  check_command rustc "Rust" "Install from: https://rustup.rs" || error "Rust is required"

  install_node_deps

  echo ""
  echo -e "  ${GREEN}${BOLD}Starting development server…${RESET}"
  echo "  • Frontend: http://localhost:1420"
  echo "  • Hot-reload enabled"
  echo "  • Press Ctrl+C to stop"
  echo ""

  npm run tauri:dev
}

# ── Web Mode ──────────────────────────────────────────────────────────────────

setup_web() {
  step "Checking requirements for Web mode"
  check_command node "Node.js" "Install from: https://nodejs.org" || {
    if [[ "$(uname)" == "Linux" ]]; then
      curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
      export NVM_DIR="$HOME/.nvm"
      [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
      nvm install --lts && nvm use --lts
    else
      error "Please install Node.js from https://nodejs.org and re-run this script"
    fi
  }

  install_node_deps

  step "Building web assets"
  npm run build
  info "Frontend built to dist/"

  # Check if Docker is available for backend
  if command -v docker &>/dev/null && command -v docker-compose &>/dev/null; then
    step "Starting backend server with Docker Compose…"
    docker-compose up -d
    info "Backend started (waiting 5 seconds for initialization)"
    sleep 5
    
    # Verify backend is running
    if curl -s http://localhost:3000/health >/dev/null 2>&1; then
      info "Backend health check passed"
    else
      warning "Backend health check failed - it may still be starting"
    fi
  else
    warning "Docker not found - backend will not be started"
    warning "The web app requires the Rust backend to be running on http://localhost:3000"
    echo ""
    echo "  To start the backend, use:"
    echo "    docker-compose up"
    echo ""
  fi

  # Install serve if not available
  if ! command -v serve &>/dev/null; then
    step "Installing 'serve' for static file serving"
    npm install -g serve 2>/dev/null || npx --yes serve --version >/dev/null
  fi

  local PORT="${PORT:-8080}"

  echo ""
  echo -e "  ${GREEN}${BOLD}✓ Web build complete!${RESET}"
  echo ""
  echo "  Starting web server on port ${PORT}…"
  echo "  Access the app at: http://localhost:${PORT}"
  echo "  Press Ctrl+C to stop"
  echo ""
  echo "  Backend API: http://localhost:3000"
  echo "  Default login: admin / Admin@123"
  echo ""

  if command -v serve &>/dev/null; then
    serve dist -l "$PORT"
  else
    npx serve dist -l "$PORT"
  fi
}

# ── Web Dev Mode ──────────────────────────────────────────────────────────────

setup_dev_web() {
  step "Checking development requirements"
  check_command node "Node.js" "Install from: https://nodejs.org" || error "Node.js is required"

  install_node_deps

  # Check if Docker is available for backend
  if command -v docker &>/dev/null && command -v docker-compose &>/dev/null; then
    step "Starting backend server with Docker Compose…"
    docker-compose up -d
    info "Backend started (waiting 5 seconds for initialization)"
    sleep 5
    
    # Verify backend is running
    if curl -s http://localhost:3000/health >/dev/null 2>&1; then
      info "Backend health check passed"
    else
      warning "Backend health check failed - it may still be starting"
    fi
  else
    warning "Docker not found - backend will not be started"
    warning "The web app requires the Rust backend to be running on http://localhost:3000"
    echo ""
    echo "  To start the backend, use:"
    echo "    docker-compose up"
    echo ""
  fi

  echo ""
  echo -e "  ${GREEN}${BOLD}Starting web development server…${RESET}"
  echo "  • URL: http://localhost:1420"
  echo "  • Hot-reload enabled"
  echo "  • Backend API: http://localhost:3000"
  echo "  • Press Ctrl+C to stop"
  echo ""

  npm run dev
}

# ── Main ──────────────────────────────────────────────────────────────────────

banner

case "$MODE" in
  desktop)   setup_desktop ;;
  dev)       setup_dev ;;
  web)       setup_web ;;
  dev-web)   setup_dev_web ;;
  help|--help|-h)
    echo "  Usage: ./setup.sh [MODE]"
    echo ""
    echo "  Modes:"
    echo "    desktop   Build and install desktop app (default)"
    echo "    dev       Run in desktop development mode (hot-reload)"
    echo "    web       Build and serve as web application"
    echo "    dev-web   Run in web development mode (hot-reload)"
    echo ""
    ;;
  *)
    error "Unknown mode: $MODE. Use: desktop | dev | web | dev-web"
    ;;
esac
