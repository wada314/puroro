#!/usr/bin/env bash
# Idempotent Cloud Agent install for the puroro multi-repo workspace.
# Dependency refresh only: toolchains, protoc, and cargo registry caches.
set -euo pipefail

# 1.83.0 is the base image default and still builds allocator-api2.
# 1.88.0 is the newest stable before dangerous_implicit_autorefs became
# deny-by-default in 1.89, and it is new enough for edition 2024.
# nightly is required by puroro feature gates and by protoc-plugin-by-closure
# cargo bindeps.
rustup toolchain install 1.83.0
rustup toolchain install 1.88.0
rustup toolchain install nightly
rustup default 1.83.0

if ! protoc --version 2>/dev/null | grep -q 'libprotoc 24.4'; then
    tmp="$(mktemp -d)"
    curl -fsSL -o "$tmp/protoc.zip" \
        https://github.com/protocolbuffers/protobuf/releases/download/v24.4/protoc-24.4-linux-x86_64.zip
    sudo unzip -o "$tmp/protoc.zip" -d /usr/local bin/protoc 'include/*'
    rm -rf "$tmp"
fi

locate_root() {
    local candidate
    for candidate in /agent/repos /workspace /workspace/repos "$PWD" "$PWD/.."; do
        if [ -d "$candidate/puroro" ] \
            && [ -d "$candidate/protobuf-core" ] \
            && [ -d "$candidate/allocator-api2" ] \
            && [ -d "$candidate/protoc-plugin-by-closure" ] \
            && [ -d "$candidate/unmanaged" ]; then
            (cd "$candidate" && pwd)
            return 0
        fi
    done
    echo "could not locate the five workspace repositories" >&2
    return 1
}

ROOT="$(locate_root)"

rustup override set 1.83.0 --path "$ROOT/allocator-api2"
rustup override set 1.83.0 --path "$ROOT/unmanaged"
rustup override set 1.88.0 --path "$ROOT/protobuf-core"
rustup override set nightly --path "$ROOT/protoc-plugin-by-closure"
rustup override set nightly --path "$ROOT/puroro"

fetch_repo() {
    local toolchain="$1"
    local dir="$2"
    if [ ! -f "$dir/Cargo.toml" ]; then
        echo "missing Cargo.toml in $dir" >&2
        return 1
    fi
    # Do not pass --locked. unmanaged's committed Cargo.lock pins
    # allocator-api2 0.4.0, which does not satisfy its manifest (^0.2),
    # so a locked fetch fails on a clean checkout.
    cargo "+$toolchain" fetch --manifest-path "$dir/Cargo.toml"
}

fetch_repo 1.83.0 "$ROOT/allocator-api2"
fetch_repo 1.83.0 "$ROOT/unmanaged"
fetch_repo 1.88.0 "$ROOT/protobuf-core"
fetch_repo nightly "$ROOT/protoc-plugin-by-closure"
fetch_repo nightly "$ROOT/puroro"

echo "install complete: root=$ROOT"
rustc +1.83.0 --version
rustc +1.88.0 --version
rustc +nightly --version
protoc --version
