#!/bin/sh
set -eu

REPOSITORY="hazyio/midway"
INSTALL_DIR=${INSTALL_DIR:-"${HOME:?HOME must be set}/.local/bin"}

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS/$ARCH" in
    Linux/x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
    Linux/aarch64 | Linux/arm64) TARGET="aarch64-unknown-linux-gnu" ;;
    Darwin/x86_64) TARGET="x86_64-apple-darwin" ;;
    Darwin/arm64 | Darwin/aarch64) TARGET="aarch64-apple-darwin" ;;
    *)
        printf 'Unsupported platform: %s/%s\n' "$OS" "$ARCH" >&2
        exit 1
        ;;
esac

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

if [ ! -f "$TEMP_DIR/midway" ]; then
    printf 'The release archive does not contain a midway executable.\n' >&2
    exit 1
fi

mkdir -p "$INSTALL_DIR"
STAGED_BINARY="$INSTALL_DIR/.midway.$$"
cp "$TEMP_DIR/midway" "$STAGED_BINARY"
chmod 755 "$STAGED_BINARY"
mv "$STAGED_BINARY" "$INSTALL_DIR/midway"
STAGED_BINARY=

printf 'Installed midway to %s/midway\n' "$INSTALL_DIR"
case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *) printf 'Add %s to your PATH to run midway from any directory.\n' "$INSTALL_DIR" ;;
esac
