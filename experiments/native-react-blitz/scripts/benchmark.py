#!/usr/bin/env python3
"""Cold-process native React/Astryx measurements on this one Linux runner.
Each sample starts a new Boa context, compiles/render CSS, handles click/input,
and paints a 980x660 RGBA frame on the CPU. Peak RSS is per-process, not Tauri.
All timings are interpreted as relative, uncontrolled runner measurements."""
import json
import os
import pathlib
import re
import statistics
import subprocess
import sys
import time

binary=pathlib.Path(sys.argv[1]).resolve()
dist=pathlib.Path(sys.argv[2]).resolve()
report=pathlib.Path(sys.argv[3]).resolve()
modes={"react":"native-basic.html","astryx":"native-index.html","stress":"native-stress.html"}
total=int(os.environ.get("BENCH_SAMPLES","11"))
assert 5 <= total <= 100, total
raw=[]
def quantile(values,p):
    ordered=sorted(values)
    # Nearest-rank percentile, deliberately conservative for small n.
    return ordered[max(0,min(len(ordered)-1,(__import__("math").ceil(len(ordered)*p)-1)))]
for mode,html in modes.items():
    target=dist/html
    if not target.is_file(): raise RuntimeError("Missing HTML "+str(target))
    for n in range(total+2):
        before=time.perf_counter()
        run=subprocess.run(
            ["/usr/bin/time","-f","MAX_RSS_KIB=%M CPU_USER_S=%U CPU_SYS_S=%S",
             str(binary),"bench-"+mode,str(target)],
            stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,timeout=45)
        wall_ms=(time.perf_counter()-before)*1000
        if run.returncode:
            print(run.stdout,run.stderr,file=sys.stderr)
            raise RuntimeError(f"{mode} fresh process failed at sample {n}")
        match=re.search(r'^BENCH_SAMPLE (.*)$',run.stdout,re.M)
        if not match: raise RuntimeError("Missing BENCH_SAMPLE "+run.stdout)
        sample={"mode":mode,"warmup":n<2,"sample_index":n,"process_wall_ms":wall_ms}
        for token in match.group(1).split():
            k,v=token.split("=")
            sample[k]=float(v) if "." in v else int(v) if v.isdigit() else v
        peak=re.search(r'MAX_RSS_KIB=(\d+) CPU_USER_S=([\d.]+) CPU_SYS_S=([\d.]+)',run.stderr)
        if not peak:raise RuntimeError("Missing resource stats "+run.stderr)
        sample["max_rss_mib"]=int(peak.group(1))/1024
        sample["cpu_s"]=float(peak.group(2))+float(peak.group(3))
        raw.append(sample)
summary={}
metrics=["mount_ms","click_ms","layout_ms","input_ms","paint_ms","process_wall_ms","max_rss_mib","cpu_s"]
for mode in modes:
    rows=[x for x in raw if x["mode"]==mode and not x["warmup"]]
    summary[mode]={"samples":len(rows)}
    for metric in metrics:
        v=[float(x[metric]) for x in rows]
        summary[mode][metric]={"p50":round(statistics.median(v),3),
                               "p95":round(quantile(v,.95),3),
                               "min":round(min(v),3),"max":round(max(v),3)}
    summary[mode]["nonwhite_ratio"]=round(statistics.median(x["nonwhite_ratio"] for x in rows),5)
    summary[mode]["author_stylesheets"]=min(x["stylesheets"] for x in rows)
output={"benchmark":"Boa/Blitz native React DOM, Linux x86_64 GitHub hosted runner",
    "note":"Not Tauri/WebView comparison; launch and styles are in-memory, renderer CPU-only. Cache warmed by earlier samples.",
    "native_blitz_commit":"fb8d6c15e9ce062b3b41e71cd48988f0093506cc",
    "samples":raw,"summary":summary}
report.parent.mkdir(parents=True,exist_ok=True)
report.write_text(json.dumps(output,indent=2)+"\n")
print("BENCHMARK_RESULTS_BEGIN")
for mode,values in summary.items():
    print(mode, "n=",values["samples"],
        "mount_p50_ms=",values["mount_ms"]["p50"],"mount_p95_ms=",values["mount_ms"]["p95"],
        "click_p50_ms=",values["click_ms"]["p50"],"input_p50_ms=",values["input_ms"]["p50"],
        "paint_p50_ms=",values["paint_ms"]["p50"],"rss_p50_mib=",values["max_rss_mib"]["p50"],
        "wall_p50_ms=",values["process_wall_ms"]["p50"],
        "stylesheets=",values["author_stylesheets"],
        "nonwhite=",values["nonwhite_ratio"])
print("BENCHMARK_RESULTS_END")
