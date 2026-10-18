#!/usr/bin/env bash
set -euo pipefail
mode="$1"
here="$(cd "$(dirname "$0")/.." && pwd)"
checkout="$here/.cache/blitz"
pinned="fb8d6c15e9ce062b3b41e71cd48988f0093506cc"
command -v git >/dev/null || { echo "Git is required" >&2; exit 2; }
command -v cargo >/dev/null || { echo "Cargo is required" >&2; exit 2; }
mkdir -p "$here/.cache"
if [ ! -d "$checkout/.git" ]; then git clone --no-checkout https://github.com/DioxusLabs/blitz.git "$checkout"; fi
if [ "$(git -C "$checkout" rev-parse HEAD 2>/dev/null || true)" != "$pinned" ]; then
  git -C "$checkout" fetch --depth 1 origin "$pinned"
  git -C "$checkout" checkout --detach "$pinned"
fi
test "$(git -C "$checkout" rev-parse HEAD)" = "$pinned"
case "$mode" in
  preact|headless-preact) html="$checkout/examples/preact/index.html" ;;
  astryx|headless-astryx)
    command -v bun >/dev/null || { echo "Bun required" >&2; exit 2; }
    (cd "$here" && bun install && bun run typecheck && bun run build)
    html="$here/dist/index.html" ;;
  *) echo "Usage: $0 preact|astryx|headless-preact|headless-astryx" >&2; exit 2 ;;
esac
echo "Blitz revision: $pinned"
echo "Probe HTML: $html"
case "$mode" in
  headless-*)
    variant="$(printf '%s' "$mode" | sed 's/^headless-//')"
    cargo run --manifest-path "$here/headless/Cargo.toml" --release -- "$variant" "$html"
    ;;
  *)
    echo "Manual GUI test; inspect layout, typography, focus, input, interaction."
    (cd "$checkout" && cargo run --locked --example preact_script -- "$html")
    ;;
esac
