#!/usr/bin/env bash
set -euo pipefail

mode="$1"
here="$(cd "$(dirname "$0")/.." && pwd)"
checkout="$here/.cache/blitz"
pinned="fb8d6c15e9ce062b3b41e71cd48988f0093506cc"

command -v git >/dev/null || { echo "Git is required" >&2; exit 2; }
command -v cargo >/dev/null || { echo "Cargo is required" >&2; exit 2; }
mkdir -p "$here/.cache"
if [ ! -d "$checkout/.git" ]; then
  git clone --no-checkout https://github.com/DioxusLabs/blitz.git "$checkout"
fi
if [ "$(git -C "$checkout" rev-parse HEAD 2>/dev/null || true)" != "$pinned" ]; then
  git -C "$checkout" fetch --depth 1 origin "$pinned"
  git -C "$checkout" checkout --detach "$pinned"
fi
test "$(git -C "$checkout" rev-parse HEAD)" = "$pinned"

case "$mode" in
  preact)
    html="$checkout/examples/preact/index.html"
    ;;
  astryx)
    command -v bun >/dev/null || { echo "Bun is required for the Astryx probe" >&2; exit 2; }
    (cd "$here" && bun install && bun run build)
    html="$here/dist/index.html"
    ;;
  *)
    echo "Usage: $0 preact|astryx" >&2
    exit 2
    ;;
esac

echo "Upstream Blitz commit: $pinned"
echo "HTML: $html"
echo "A launched window is not a compatibility result: exercise click, input, style and focus."
(cd "$checkout" && cargo run --locked --example preact_script -- "$html")
