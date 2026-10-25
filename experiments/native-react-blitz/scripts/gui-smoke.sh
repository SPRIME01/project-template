#!/usr/bin/env bash
set -euo pipefail
html="$(realpath "$1")"
log="$2"
out="$3"
test -s "$html"
command -v xvfb-run >/dev/null
command -v xdotool >/dev/null
command -v import >/dev/null
command -v identify >/dev/null
# Force Mesa lavapipe software Vulkan on CI/Xvfb. OpenGL lacks a surface
# for Blitz's Vello/WGPU window renderer on this runner.
export WGPU_BACKEND=vulkan LIBGL_ALWAYS_SOFTWARE=1
if [ -f /usr/share/vulkan/icd.d/lvp_icd.x86_64.json ]; then
  export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json
fi
vulkaninfo --summary | head -n 40 || true
xvfb-run -a -s "-screen 0 1280x800x24" bash -c '
  set -euo pipefail
  html="$1"; log="$2"; screenshot="$3"
  openbox > /tmp/blitz-openbox.log 2>&1 &
  wm=$!
  cleanup() { kill "$child" "$wm" 2>/dev/null || true; wait "$child" 2>/dev/null || true; }
  trap cleanup EXIT
  cargo run --locked --example preact_script -- "$html" > "$log" 2>&1 &
  child=$!
  ready=0
  for attempt in $(seq 1 40); do
    if ! kill -0 "$child" 2>/dev/null; then tail -n 100 "$log"; exit 1; fi
    if xdotool search --onlyvisible --name "React|Blitz|Preact|Todo|Native" > /tmp/blitz-windows.txt 2>/dev/null; then
      ready=1; break
    fi
    sleep 1
  done
  if [ "$ready" != 1 ]; then
    xwininfo -root -tree || true
    tail -n 100 "$log"
    exit 1
  fi
  sleep 2
  import -window root "$screenshot"
  test -s "$screenshot"
  echo "BLITZ_NATIVE_WINDOW_CAPTURED"
  identify "$screenshot"
' bash "$html" "$log" "$out"
colors="$(identify -format '%k' "$out")"
echo "BLITZ_GUI_UNIQUE_COLORS=$colors"
test "$colors" -gt 5 || { echo "Screen appears blank/uniform" >&2; exit 1; }
