#!/usr/bin/env python3
"""Preserve the actual Vite bundle but inline CSS for offline Blitz DocumentConfig.
Blitz's in-memory ScriptDocument does not use the browser network provider.
Never inline JS: keep the real Vite output and ES module resolution."""
from pathlib import Path
import re
import sys

root=Path(sys.argv[1]).resolve()
links=0
for source in ("index.html","basic.html","stress.html","stress-memo.html"):
    html=(root/source).read_text()
    def replace(m):
        global links
        tag=m.group(0)
        if not re.search(r'\brel=["\']stylesheet["\']',tag):
            return tag
        href=re.search(r'\bhref=["\']([^"\']+)["\']',tag)
        if not href:
            raise ValueError(f"Stylesheet link missing href: {tag}")
        asset=(root/href.group(1)).resolve()
        if not asset.is_relative_to(root) or not asset.is_file():
            raise ValueError(f"unsafe/missing CSS path {href.group(1)}")
        links+=1
        css=asset.read_text()
        return '<style data-native-asset="'+asset.name+'">'+css+'</style>'
    updated=re.sub(r'<link\b[^>]*>',replace,html)
    target=root/("native-"+source)
    target.write_text(updated)
    print("EMBED_CSS",target, "stylesheets",updated.count('data-native-asset='))
if links==0:
    raise SystemExit("FAIL: No external CSS found. Astryx styles not tested.")
