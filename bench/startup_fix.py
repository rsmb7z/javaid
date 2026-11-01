#!/usr/bin/env python3
"""Does a JVM startup fix close the gap? Measures the best available ones.

Block A  one file (latency): Checkstyle and PMD on JDK 21 as shipped, on JDK 25, and on JDK 25 with a
         trained AOT cache (Project Leyden: JEP 483 class loading/linking + JEP 515 method profiles),
         with and without latency-oriented JIT/GC flags. javaid is the reference.
Block B  whole repository (Guava): the same startup fixes, to show whether they matter when
         the run is dominated by parsing and analysis rather than by startup.
Block C  a warm resident JVM: Checkstyle and PMD linting the same file repeatedly inside one process
         (what an IDE plugin or a daemon gives you), with the process's peak resident memory.

usage: startup_fix.py --scratch DIR --javaid BIN --jdk25 JAVA_HOME --out DIR [--blocks ABC]
"""
import argparse
import glob
import json
import os
import statistics
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))


def measure(cmd, out_path=os.devnull, err_path=os.devnull):
    with open(out_path, "wb") as so, open(err_path, "wb") as se:
        t0 = time.perf_counter()
        p = subprocess.Popen(cmd, stdout=so, stderr=se)
        _, status, ru = os.wait4(p.pid, 0)
        wall = time.perf_counter() - t0
    p.returncode = os.waitstatus_to_exitcode(status)
    return {"wall": wall, "rss_mb": ru.ru_maxrss / 1024.0, "cpu": ru.ru_utime + ru.ru_stime, "rc": p.returncode}


def summarize(samples):
    w = [s["wall"] for s in samples]
    return {
        "wall_median": statistics.median(w),
        "wall_min": min(w),
        "wall_max": max(w),
        "rss_mb": statistics.median(s["rss_mb"] for s in samples),
        "cpu": statistics.median(s["cpu"] for s in samples),
        "rcs": sorted({s["rc"] for s in samples}),
    }


def fmt(s):
    t = lambda x: f"{x * 1000:.1f} ms" if x < 0.1 else f"{x:.2f} s"
    return f"{t(s['wall_median'])} ({t(s['wall_min'])} – {t(s['wall_max'])})"


class Tools:
    def __init__(self, a):
        self.a = a
        self.java21 = "java"
        self.java25 = os.path.join(a.jdk25, "bin", "java")
        self.cs_jar = os.path.join(a.scratch, "tools/checkstyle/checkstyle-14.4.0-all.jar")
        pmd_lib = os.path.join(a.scratch, "tools/pmd/pmd-bin-7.29.0/lib")
        # An explicit, stable classpath: the AOT cache must be used with the classpath it was built with.
        self.pmd_cp = ":".join(sorted(glob.glob(os.path.join(pmd_lib, "*.jar"))))
        self.cs_cfg = os.path.join(HERE, "checkstyle-spike.xml")
        self.pmd_cfg = os.path.join(HERE, "pmd-spike.xml")
        self.cache_dir = os.path.join(a.scratch, "results", "aot")
        os.makedirs(self.cache_dir, exist_ok=True)

    def cmd(self, tool, java, jvm_flags, target, pmd_threads="1"):
        if tool == "checkstyle":
            return [java] + jvm_flags + ["-jar", self.cs_jar, "-c", self.cs_cfg, target]
        return [java] + jvm_flags + ["-cp", self.pmd_cp, "net.sourceforge.pmd.cli.PmdCli", "check", "-d", target,
                                      "-R", self.pmd_cfg, "-f", "text", "--no-cache", "--no-progress", "-t", pmd_threads]

    def build_aot_cache(self, tool, train_dir, pmd_threads="1"):
        """Training run on a small, different directory; the cache is written when the JVM exits."""
        path = os.path.join(self.cache_dir, f"{tool}-t{pmd_threads}.aot" if tool == "pmd" else f"{tool}.aot")
        if os.path.exists(path):
            os.remove(path)
        cmd = self.cmd(tool, self.java25, [f"-XX:AOTCacheOutput={path}"], train_dir, pmd_threads)
        t0 = time.perf_counter()
        r = subprocess.run(cmd, capture_output=True, text=True)
        took = time.perf_counter() - t0
        if not os.path.exists(path):
            sys.exit(f"AOT cache was not created for {tool}: rc={r.returncode}\n{r.stderr[:1500]}")
        return path, {"cache_mb": os.path.getsize(path) / 1e6, "training_seconds": took}


def block_a(t, a, out):
    target = os.path.join(a.scratch, "corpus", "latency-one-file")
    train = os.path.join(a.scratch, "corpus", "guava", "guava", "src", "com", "google", "common", "primitives")
    rows = []
    for tool in ("checkstyle", "pmd"):
        cache, info = t.build_aot_cache(tool, train)
        print(f"[A] {tool}: AOT cache {info['cache_mb']:.0f} MB, training run {info['training_seconds']:.1f} s", flush=True)
        variants = [
            ("JDK 21 as shipped", t.java21, []),
            ("JDK 25", t.java25, []),
            ("JDK 25 + AOT cache", t.java25, [f"-XX:AOTCache={cache}"]),
            ("JDK 25 + AOT cache + C1-only JIT + SerialGC", t.java25, [f"-XX:AOTCache={cache}", "-XX:TieredStopAtLevel=1", "-XX:+UseSerialGC"]),
        ]
        for label, java, flags in variants:
            cmd = t.cmd(tool, java, flags, target)
            measure(cmd)  # one discarded warm-up
            samples = [measure(cmd, err_path=os.path.join(t.cache_dir, "last.err")) for _ in range(a.latency_runs)]
            warn = open(os.path.join(t.cache_dir, "last.err"), errors="replace").read()
            s = summarize(samples)
            s.update(tool=tool, variant=label, cache_warning=("warning" in warn.lower() and "aot" in warn.lower()))
            rows.append(s)
            print(f"[A] {tool:10} {label:48} {fmt(s)}  rss {s['rss_mb']:.0f} MB", flush=True)
    jv = [measure([a.javaid, target]) for _ in range(a.latency_runs * 3)]
    s = summarize(jv)
    s.update(tool="javaid", variant="native binary", cache_warning=False)
    rows.append(s)
    print(f"[A] javaid     {'native binary':48} {fmt(s)}  rss {s['rss_mb']:.0f} MB", flush=True)
    out["A"] = rows


def block_b(t, a, out):
    target = os.path.join(a.scratch, "corpus", "guava")
    train = os.path.join(target, "guava", "src", "com", "google", "common", "primitives")
    rows = []
    plan = []
    cs_cache, _ = t.build_aot_cache("checkstyle", train)
    plan.append(("checkstyle", "Checkstyle, JDK 21 as shipped", t.cmd("checkstyle", t.java21, [], target)))
    plan.append(("checkstyle", "Checkstyle, JDK 25 + AOT cache", t.cmd("checkstyle", t.java25, [f"-XX:AOTCache={cs_cache}"], target)))
    for threads, tag in (("1", "1 thread"), ("16", "16 threads")):
        cache, _ = t.build_aot_cache("pmd", train, threads)
        plan.append(("pmd", f"PMD {tag}, JDK 21 as shipped", t.cmd("pmd", t.java21, [], target, threads)))
        plan.append(("pmd", f"PMD {tag}, JDK 25 + AOT cache", t.cmd("pmd", t.java25, [f"-XX:AOTCache={cache}"], target, threads)))
    plan.append(("javaid", "javaid, 1 thread", [a.javaid, "-j", "1", target]))
    plan.append(("javaid", "javaid, 16 threads", [a.javaid, target]))
    for tool, label, cmd in plan:
        runs = 10 if tool == "javaid" else a.repo_runs
        if tool == "javaid":
            measure(cmd)
        samples = [measure(cmd) for _ in range(runs)]
        s = summarize(samples)
        s.update(tool=tool, variant=label)
        rows.append(s)
        print(f"[B] {label:36} {fmt(s)}  rss {s['rss_mb']:.0f} MB  cpu {s['cpu']:.0f} s", flush=True)
    out["B"] = rows


def block_c(t, a, out):
    # Each tool gets its own JVM and classpath: the two distributions bundle Saxon with different
    # jar signatures and cannot share a classloader.
    javac = os.path.join(a.jdk25, "bin", "javac")
    target = os.path.join(a.scratch, "corpus", "latency-one-file", "Joiner.java")
    rows = []
    for mode, cls, cp, cfg, iters in (
        ("checkstyle", "WarmCheckstyle", t.cs_jar, t.cs_cfg, 400),
        ("pmd", "WarmPmd", t.pmd_cp, t.pmd_cfg, 150),
    ):
        classes = os.path.join(a.scratch, "results", f"warm-classes-{mode}")
        os.makedirs(classes, exist_ok=True)
        r = subprocess.run([javac, "--release", "21", "-cp", cp, "-d", classes, os.path.join(HERE, cls + ".java")], capture_output=True, text=True)
        if r.returncode != 0:
            sys.exit(f"{cls}.java did not compile:\n" + r.stderr[:2500])
        res = subprocess.run([t.java25, "-cp", f"{classes}:{cp}", cls, cfg, target, str(iters)], capture_output=True, text=True)
        if "ERROR" in res.stderr:
            sys.exit(f"{mode}: the tool reported errors (results would be invalid):\n" + res.stderr[:1500])
        line = (res.stdout.strip().splitlines() or [res.stderr[:300]])[-1]
        rows.append({"mode": mode, "line": line})
        print(f"[C] {line}", flush=True)
    out["C"] = rows


def write_markdown(out, path):
    L = ["# Does a JVM startup fix close the gap?", ""]
    if "A" in out:
        L += ["## A. One 21.5 KB file (what a pre-commit hook, editor save or agent hook pays)", "",
              "| tool | configuration | wall median (min – max) | peak RSS |", "|---|---|---:|---:|"]
        for r in out["A"]:
            L.append(f"| {r['tool']} | {r['variant']} | {fmt(r)} | {r['rss_mb']:.0f} MB |")
        L.append("")
    if "B" in out:
        L += ["## B. Guava, whole repository (3,277 files, 29.5 MB)", "",
              "| configuration | wall median (min – max) | peak RSS | CPU time |", "|---|---:|---:|---:|"]
        for r in out["B"]:
            L.append(f"| {r['variant']} | {fmt(r)} | {r['rss_mb']:.0f} MB | {r['cpu']:.0f} s |")
        L.append("")
    if "C" in out:
        L += ["## C. Warm resident JVM (same file, repeated in one process)", "", "```"] + [r["line"] for r in out["C"]] + ["```", ""]
    open(path, "w").write("\n".join(L))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--scratch", required=True)
    ap.add_argument("--javaid", required=True)
    ap.add_argument("--jdk25", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--blocks", default="ABC")
    ap.add_argument("--latency-runs", type=int, default=10)
    ap.add_argument("--repo-runs", type=int, default=3)
    a = ap.parse_args()
    os.makedirs(a.out, exist_ok=True)
    t = Tools(a)
    out = {}
    for name, fn in (("A", block_a), ("B", block_b), ("C", block_c)):
        if name in a.blocks:
            fn(t, a, out)
            json.dump(out, open(os.path.join(a.out, "results.json"), "w"), indent=1)
            write_markdown(out, os.path.join(a.out, "RESULTS.md"))
    print("STARTUP-FIX-DONE", flush=True)


if __name__ == "__main__":
    sys.exit(main())
