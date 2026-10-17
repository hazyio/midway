#!/bin/sh
set -eu

MODE=install
if [ "$#" -gt 1 ] || { [ "$#" -eq 1 ] && [ "$1" != "update" ]; }; then
    printf 'Usage: sh install.sh [update]\n' >&2
    exit 2
fi
if [ "$#" -eq 1 ]; then
    MODE=update
fi

REPOSITORY="hazyio/midway"

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS/$ARCH" in
    Linux/x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
    Darwin/arm64 | Darwin/aarch64) TARGET="aarch64-apple-darwin" ;;
    Linux/aarch64 | Linux/arm64 | Darwin/x86_64)
        printf 'No prebuilt Midway release is available for %s/%s. Install Rust and Cargo, then run: cargo install midway-cli\n' "$OS" "$ARCH" >&2
        exit 1
        ;;
    *)
        printf 'Unsupported platform: %s/%s. See https://midway.hazyio.com/installation for supported builds.\n' "$OS" "$ARCH" >&2
        exit 1
        ;;
esac

if [ "$MODE" = "update" ]; then
    printf 'Checking for Midway updates...\n'
    if [ -n "${INSTALL_DIR:-}" ]; then
        if [ ! -d "$INSTALL_DIR" ]; then
            printf 'Install directory does not exist: %s\n' "$INSTALL_DIR" >&2
            exit 1
        fi
        INSTALL_DIR=$(CDPATH= cd -- "$INSTALL_DIR" && pwd)
        INSTALLED_BIN="$INSTALL_DIR/midway"
    else
        if ! INSTALLED_BIN=$(command -v midway); then
            printf "Midway is not installed. Run 'sh install.sh' to install it first.\n" >&2
            exit 1
        fi
        case "$INSTALLED_BIN" in
            /*) ;;
            *) INSTALLED_BIN="$(pwd)/$INSTALLED_BIN" ;;
        esac
        INSTALL_DIR=$(dirname "$INSTALLED_BIN")
    fi
    if [ ! -x "$INSTALLED_BIN" ]; then
        printf 'Midway was not found at %s.\n' "$INSTALLED_BIN" >&2
        exit 1
    fi
    if ! CURRENT_VERSION=$("$INSTALLED_BIN" --version); then
        printf 'Could not read the installed Midway version from %s.\n' "$INSTALLED_BIN" >&2
        exit 1
    fi
else
    INSTALL_DIR=${INSTALL_DIR:-"${HOME:?HOME must be set}/.local/bin"}
fi

if ! command -v curl >/dev/null 2>&1; then
    printf 'curl is required to install Midway.\n' >&2
    exit 1
fi

if ! command -v tar >/dev/null 2>&1; then
    printf 'tar is required to install Midway.\n' >&2
    exit 1
fi

ASSET="midway-${TARGET}.tar.gz"
URL="https://github.com/${REPOSITORY}/releases/latest/download/${ASSET}"
TEMP_DIR=$(mktemp -d)
STAGED_BINARY=
cleanup() {
    rm -rf "$TEMP_DIR"
    if [ -n "$STAGED_BINARY" ]; then
        rm -f "$STAGED_BINARY"
    fi
}
trap cleanup 0 HUP INT TERM

if ! curl --fail --location --silent --show-error "$URL" -o "$TEMP_DIR/$ASSET"; then
    printf 'Could not download %s. Check that a release with this asset exists:\n%s\n' "$ASSET" "$URL" >&2
    exit 1
fi

if ! tar -xzf "$TEMP_DIR/$ASSET" -C "$TEMP_DIR" midway; then
    printf 'The release archive must contain a top-level midway executable.\n' >&2
    exit 1
fi

if [ ! -x "$TEMP_DIR/midway" ]; then
    printf 'The release archive does not contain a midway executable.\n' >&2
    exit 1
fi

if ! RELEASE_VERSION=$("$TEMP_DIR/midway" --version); then
    printf 'Could not read the Midway version from the downloaded release.\n' >&2
    exit 1
fi

if [ "$MODE" = "update" ]; then
    if [ "$CURRENT_VERSION" = "$RELEASE_VERSION" ]; then
        printf 'Midway is up to date (%s). Exiting.\n' "$CURRENT_VERSION"
        exit 0
    fi
    printf 'Updating Midway from %s to %s...\n' "$CURRENT_VERSION" "$RELEASE_VERSION"
fi

mkdir -p "$INSTALL_DIR"
STAGED_BINARY="$INSTALL_DIR/.midway.$$"
cp "$TEMP_DIR/midway" "$STAGED_BINARY"
chmod 755 "$STAGED_BINARY"
mv "$STAGED_BINARY" "$INSTALL_DIR/midway"
STAGED_BINARY=

if [ "$MODE" = "update" ]; then
    printf 'Updated midway to %s/midway\n' "$INSTALL_DIR"
else
    printf 'Installed midway to %s/midway\n' "$INSTALL_DIR"
fi
case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *) printf 'Add %s to your PATH to run midway from any directory.\n' "$INSTALL_DIR" ;;
esac
