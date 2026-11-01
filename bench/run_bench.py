#!/usr/bin/env python3
"""Benchmark javaid against Checkstyle and PMD on identical files and equivalent rules.

Wall time, peak RSS and CPU time come from the kernel's accounting for the child process
(wait4), not from in-process timers. Every tool writes its findings to a file, so output
cost is paid by all of them. Each scenario gets one discarded warm-up run, then N samples.

usage: run_bench.py --scratch DIR --javaid BIN --out DIR --corpus guava --corpus spring-framework
"""
import argparse
import json
import os
import platform
import shutil
import statistics
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))


def sh(cmd):
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        return (r.stdout or r.stderr).strip()
    except Exception as e:  # noqa: BLE001 - best-effort environment capture
        return f"<{e}>"


def measure(cmd, env, out_path, err_path):
    full_env = dict(os.environ)
    full_env.update(env or {})
    with open(out_path, "wb") as so, open(err_path, "wb") as se:
        t0 = time.perf_counter()
        p = subprocess.Popen(cmd, stdout=so, stderr=se, env=full_env)
        _, status, ru = os.wait4(p.pid, 0)
        wall = time.perf_counter() - t0
    p.returncode = os.waitstatus_to_exitcode(status)
    return {"wall": wall, "rss_mb": ru.ru_maxrss / 1024.0, "cpu": ru.ru_utime + ru.ru_stime, "rc": p.returncode}


def build_scenarios(a, target):
    java = ["java"]
    cs = java + ["-jar", a.checkstyle]
    jv = a.javaid
    cs_cfg = a.cs_config or os.path.join(HERE, "checkstyle-spike.xml")
    jv_cfg = ["--config", a.javaid_config] if a.javaid_config else []
    S = []

    def add(key, label, tool, kind, cmd, env=None, role=None):
        S.append({"key": key, "label": label, "tool": tool, "kind": kind, "cmd": cmd, "env": env or {}, "role": role})

    # javaid
    add("jv-parse-j1", "javaid parse only, 1 thread", "javaid", "fast", [jv, *jv_cfg, "--parse-only", "-j", "1", target])
    add("jv-lint-j1", f"javaid {a.rules_label}, 1 thread", "javaid", "fast", [jv, *jv_cfg, "-j", "1", target], role="jv1")
    add("jv-lint-j8", f"javaid {a.rules_label}, 8 threads", "javaid", "fast", [jv, *jv_cfg, "-j", "8", target])
    add("jv-lint-jall", f"javaid {a.rules_label}, all threads", "javaid", "fast", [jv, *jv_cfg, target], role="jvN")

    # Checkstyle (single-threaded)
    add("cs-floor", "Checkstyle 1 trivial rule", "checkstyle", "slow", cs + ["-c", os.path.join(HERE, "checkstyle-floor.xml"), target])
    add("cs-spike", f"Checkstyle {a.rules_label}, default JVM", "checkstyle", "slow", cs + ["-c", cs_cfg, target], role="cs")
    add(
        "cs-spike-tuned",
        f"Checkstyle {a.rules_label}, tuned JVM",
        "checkstyle",
        "slow",
        java + a.tuned_single.split() + ["-jar", a.checkstyle, "-c", cs_cfg, target],
        role="cs_tuned",
    )

    # PMD (default is one thread per core)
    pmd = [a.pmd, "check", "-d", target, "-f", "text", "--no-cache", "--no-progress"]
    spike = ["-R", os.path.join(HERE, "pmd-spike.xml")]
    add("pmd-floor-t1", "PMD 1 rule, 1 thread", "pmd", "slow", pmd + ["-R", os.path.join(HERE, "pmd-floor.xml"), "-t", "1"])
    add("pmd-spike-t1", "PMD 5 rules, 1 thread, default JVM", "pmd", "slow", pmd + spike + ["-t", "1"], role="pmd1")
    add("pmd-spike-t1-tuned", "PMD 5 rules, 1 thread, tuned JVM", "pmd", "slow", pmd + spike + ["-t", "1"], {"PMD_JAVA_OPTS": a.tuned_single})
    add("pmd-spike-default", "PMD 5 rules, default threads, default JVM", "pmd", "slow", pmd + spike, role="pmdN")
    add("pmd-spike-tuned", "PMD 5 rules, default threads, fastest JVM flags", "pmd", "slow", pmd + spike, {"PMD_JAVA_OPTS": a.tuned_multi})
    add("pmd-spike-lowmem", "PMD 5 rules, default threads, low-memory JVM flags", "pmd", "slow", pmd + spike, {"PMD_JAVA_OPTS": a.lowmem_multi})

    if a.ci_cores:
        # Emulate a small CI runner: pin every tool to the first N logical CPUs (distinct
        # physical cores on this machine). The JVM sizes its thread pools from the affinity mask.
        n = a.ci_cores
        pin = ["taskset", "-c", ",".join(str(i) for i in range(n))]
        add(f"ci{n}-jv", f"[{n}-core runner] javaid", "javaid", "fast", pin + [jv, *jv_cfg, target])
        add(
            f"ci{n}-cs",
            f"[{n}-core runner] Checkstyle, tuned JVM",
            "checkstyle",
            "slow",
            pin + java + a.tuned_single.split() + ["-jar", a.checkstyle, "-c", os.path.join(HERE, "checkstyle-spike.xml"), target],
        )
        add(
            f"ci{n}-cs-default",
            f"[{n}-core runner] Checkstyle, default JVM",
            "checkstyle",
            "slow",
            pin + cs + ["-c", os.path.join(HERE, "checkstyle-spike.xml"), target],
        )
        add(f"ci{n}-pmd", f"[{n}-core runner] PMD, default threads, tuned JVM", "pmd", "slow", pin + pmd + spike, {"PMD_JAVA_OPTS": a.tuned_multi})

    if not a.skip_realistic:
        add("cs-google", "Checkstyle google_checks.xml (what teams run)", "checkstyle", "realistic", cs + ["-c", "/google_checks.xml", target])
        add("pmd-quickstart", "PMD quickstart ruleset (what teams run)", "pmd", "realistic", pmd + ["-R", "rulesets/java/quickstart.xml"])
    return S


def run_scenario(sc, runs, outdir):
    os.makedirs(outdir, exist_ok=True)
    out, err = os.path.join(outdir, sc["key"] + ".out"), os.path.join(outdir, sc["key"] + ".err")
    # One discarded warm-up for the sub-second native runs (CPU clocks, first-touch). The JVM
    # tools keep no state between runs, and the corpus is pre-read into the page cache, so a
    # warm-up there would only cost minutes.
    if sc["kind"] == "fast":
        measure(sc["cmd"], sc["env"], out, err)
    return [measure(sc["cmd"], sc["env"], out, err) for _ in range(runs)]


def prime_page_cache(target):
    subprocess.run(["bash", "-c", f"find '{target}' -name '*.java' -print0 | xargs -0 cat > /dev/null"], check=False)


def stats(samples, field):
    v = [s[field] for s in samples]
    return {"median": statistics.median(v), "min": min(v), "max": max(v)}


def env_info(a, corpora):
    info = {
        "date": time.strftime("%Y-%m-%d %H:%M:%S %Z"),
        "kernel": platform.release(),
        "cpu": sh(["bash", "-c", "lscpu | sed -n 's/^Model name: *//p'"]),
        "logical_cpus": os.cpu_count(),
        "mem": sh(["bash", "-c", "free -h | awk '/^Mem:/{print $2}'"]),
        "governor": sh(["bash", "-c", "cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null"]),
        "java": sh(["java", "-version"]).splitlines()[0:2],
        "checkstyle": os.path.basename(a.checkstyle),
        "pmd": os.path.basename(os.path.dirname(os.path.dirname(a.pmd))),  # e.g. pmd-bin-7.29.0
        "tuned_single": a.tuned_single,
        "tuned_multi": a.tuned_multi,
        "lowmem_multi": a.lowmem_multi,
        "ci_cores": a.ci_cores,
        "corpora": {},
    }
    for c in corpora:
        d = os.path.join(a.scratch, "corpus", c)
        info["corpora"][c] = {
            "commit": sh(["git", "-C", d, "rev-parse", "--short", "HEAD"]) if os.path.isdir(os.path.join(d, ".git")) else "n/a (not a git checkout)",
            "java_files": int(sh(["bash", "-c", f"find '{d}' -name '*.java' | wc -l"]) or 0),
            "bytes": int(sh(["bash", "-c", f"find '{d}' -name '*.java' -print0 | xargs -0 cat | wc -c"]) or 0),
        }
    return info


def fmt_time(seconds):
    return f"{seconds * 1000:.1f} ms" if seconds < 0.1 else f"{seconds:.2f} s"


def fmt_row(sc, r, base):
    w, m, c = r["wall"], r["rss"], r["cpu"]
    cells = [sc["label"], fmt_time(w["median"]), f"{fmt_time(w['min'])} – {fmt_time(w['max'])}", f"{m['median']:.0f}", f"{c['median']:.2f}"]
    for b in base:
        cells.append(f"{b['wall'] / w['median']:.1f}×" if b else "")
    return "| " + " | ".join(cells) + " |"


def markdown(info, results):
    lines = ["# Benchmark results", "", "```json", json.dumps(info, indent=2), "```", ""]
    for corpus, scs in results.items():
        meta = info["corpora"].get(corpus, {})
        title = f"## {corpus}"
        if meta:
            title += f" — {meta['java_files']} files, {meta['bytes'] / 1e6:.1f} MB"
        lines += [title, ""]
        by_role = {s["role"]: s["result"]["wall"]["median"] for s in scs if s.get("role")}
        lines += [
            "| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |",
            "|---|---:|---:|---:|---:|---:|---:|",
        ]
        for s in scs:
            r = s["result"]
            base = [{"wall": by_role["cs"]} if "cs" in by_role else None, {"wall": by_role["pmdN"]} if "pmdN" in by_role else None]
            lines.append(fmt_row(s, r, base))
        lines.append("")
    return "\n".join(lines)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--scratch", required=True)
    ap.add_argument("--javaid", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--corpus", action="append", required=True)
    ap.add_argument("--slow-runs", type=int, default=3)
    ap.add_argument("--fast-runs", type=int, default=10)
    ap.add_argument("--realistic-runs", type=int, default=1)
    ap.add_argument("--only", action="append", help="run only scenarios whose key contains this")
    ap.add_argument("--skip-realistic", action="store_true")
    ap.add_argument("--raw-subdir", default="formal", help="scratch/results/<this>/<corpus> holds each run's raw output; use a different one per invocation")
    ap.add_argument("--rules-label", default="5 rules", help="how the rule set is described in scenario labels")
    ap.add_argument("--cs-config", help="Checkstyle config for the Checkstyle scenarios (default: bench/checkstyle-spike.xml)")
    ap.add_argument("--javaid-config", help="checkstyle.xml to pass to javaid with --config (default: javaid's built-in defaults)")
    ap.add_argument("--ci-cores", type=int, default=0, help="also run pinned to N cores, like a small CI runner")
    ap.add_argument("--tuned-single", default="-XX:+UseSerialGC -XX:TieredStopAtLevel=1 -Xmx512m")
    ap.add_argument("--tuned-multi", default="-XX:+UseParallelGC")
    ap.add_argument("--lowmem-multi", default="-XX:+UseSerialGC")
    a = ap.parse_args()
    a.checkstyle = os.path.join(a.scratch, "tools/checkstyle/checkstyle-14.4.0-all.jar")
    a.pmd = os.path.join(a.scratch, "tools/pmd/pmd-bin-7.29.0/bin/pmd")
    os.makedirs(a.out, exist_ok=True)

    info = env_info(a, a.corpus)
    results = {}
    for corpus in a.corpus:
        target = os.path.join(a.scratch, "corpus", corpus)
        outdir = os.path.join(a.scratch, "results", a.raw_subdir, corpus)
        results[corpus] = []
        prime_page_cache(target)
        for sc in build_scenarios(a, target):
            if a.only and not any((sc["key"] == o[:-1]) if o.endswith("$") else (o in sc["key"]) for o in a.only):
                continue
            runs = {"fast": a.fast_runs, "slow": a.slow_runs, "realistic": a.realistic_runs}[sc["kind"]]
            print(f"[{corpus}] {sc['label']} ({runs} runs) ...", flush=True)
            samples = run_scenario(sc, runs, outdir)
            result = {"wall": stats(samples, "wall"), "rss": stats(samples, "rss_mb"), "cpu": stats(samples, "cpu"), "rcs": sorted({s["rc"] for s in samples}), "samples": samples}
            results[corpus].append({**{k: sc[k] for k in ("key", "label", "tool", "role")}, "result": result})
            print(f"    wall {result['wall']['median']:.2f}s  rss {result['rss']['median']:.0f}MB  cpu {result['cpu']['median']:.1f}s  rc={result['rcs']}", flush=True)
            with open(os.path.join(a.out, "results.json"), "w") as fh:
                json.dump({"env": info, "results": results}, fh, indent=1)

    with open(os.path.join(a.out, "RESULTS.md"), "w") as fh:
        fh.write(markdown(info, results))
    print("BENCH-DONE", flush=True)


if __name__ == "__main__":
    sys.exit(main())
