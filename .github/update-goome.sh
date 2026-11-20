#!/usr/bin/env bash
set -euo pipefail

latest_version() {
  curl -fsS -A "goome-app-template-rust-updater" https://crates.io/api/v1/crates/goome | jq -r .crate.max_stable_version | stable_only
}

current_version() {
  grep -m1 '^goome = ' Cargo.toml | cut -d'"' -f2
}

published() {
  curl -fsS -A "goome-app-template-rust-updater" "https://crates.io/api/v1/crates/goome/$1" > /dev/null
}

apply_pins() {
  for file in Cargo.toml variants/*/Cargo.toml; do
    [ -f "$file" ] || continue
    sed -i "s/^goome = \".*\"/goome = \"$1\"/" "$file"
  done
  if git ls-files --error-unmatch Cargo.lock > /dev/null 2>&1; then
    cargo update -p goome
  fi
}

smoke() {
  export GOOME_BACKEND=test
  if [ "${GOOME_NOSMOKE:-}" = 1 ]; then
    echo "smoke check skipped"
    return
  fi
  work=$(mktemp -d)
  cargo build
  mkdir -p "$work/shots"
  out=$(GOOME_TEST_SHOTS="$work/shots" GOOME_TEST_SCRIPT="screen 1280x720 1; 3 click 60,28; end 6" cargo run -q)
  echo "$out"
  ls "$work/shots" | grep -q ppm
  if [ -d variants ]; then echo "$out" | grep -q "overlay mode"; fi
  for v in variants/*/; do
    [ -d "$v" ] || continue
    name=$(basename "$v")
    dir="$work/v-$name"
    mkdir -p "$dir" "$work/shots-$name"
    cp -r Cargo.toml src "$dir/"
    cp -r "$v". "$dir/"
    CARGO_TARGET_DIR="$PWD/target" GOOME_TEST_SHOTS="$work/shots-$name" GOOME_TEST_SCRIPT="screen 1280x720 1; 2 hotkey alt+m; 3 click 100,140; end 6" cargo run -q --manifest-path "$dir/Cargo.toml"
    ls "$work/shots-$name" | grep -q ppm
  done
  rm -rf "$work"
  git ls-files --error-unmatch Cargo.lock > /dev/null 2>&1 || rm -f Cargo.lock
}

is_newer() {
  [ "$1" != "$2" ] && [ "$(printf '%s\n%s\n' "$1" "$2" | sort -V | tail -n1)" = "$1" ]
}

stable_only() {
  grep -E '^v?[0-9]+\.[0-9]+\.[0-9]+$' | sed 's/^v//'
}

set_identity() {
  git config user.name "$(git log -1 --format=%an)"
  git config user.email "$(git log -1 --format=%ae)"
}

commit_and_push() {
  branch=$1
  message=$2
  if [ "${GOOME_DRY:-}" = 1 ]; then
    git --no-pager diff --stat
    return
  fi
  git commit -q -a -m "$message"
  git push -q origin "$branch"
}

on_branch() {
  if [ "${GOOME_DRY:-}" != 1 ]; then
    git checkout -q "$1"
    git pull -q --ff-only origin "$1"
  fi
}

run() {
  latest=$(latest_version)
  if [ -z "$latest" ]; then
    echo "could not read the latest version"
    exit 1
  fi
  on_branch dev
  current=$(current_version)
  echo "latest $latest, pinned $current"
  if ! is_newer "$latest" "$current"; then
    echo "up to date"
    exit 0
  fi
  if ! published "$latest"; then
    echo "$latest is not available from the registry yet"
    exit 0
  fi
  set_identity
  apply_pins "$latest"
  smoke
  commit_and_push dev "Bump goome to $latest"
  if [ "${GOOME_DRY:-}" = 1 ]; then
    exit 0
  fi
  git checkout -q main
  git pull -q --ff-only origin main
  if is_newer "$latest" "$(current_version)"; then
    apply_pins "$latest"
    commit_and_push main "Update to $latest"
  fi
}

run
