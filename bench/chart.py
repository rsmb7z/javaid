#!/usr/bin/env python3
"""Bar chart of bench/results/full-configs-timing.tsv: wall seconds, median, linear axis from zero.
usage: python3 bench/chart.py   (writes docs/img/benchmark-light.png and benchmark-dark.png)"""
import collections, os, statistics as st
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
rows = collections.defaultdict(list)
for line in open(os.path.join(HERE, "results", "full-configs-timing.tsv")):
    p = line.rstrip("\n").split("\t")
    if len(p) == 4 and not p[0].startswith("joiner"):
        rows[p[0]].append(float(p[1]))
med = {k: st.median(v) for k, v in rows.items()}
groups = [("Google\nGuava", "guava/google"), ("Google\nSpring", "spring-framework/google"),
          ("Sun\nGuava", "guava/sun"), ("Sun\nSpring", "spring-framework/sun")]
series = [("Checkstyle 14.4.0", "checkstyle", "#8b8b8b"), ("javaid, 1 thread", "javaid-j1", "#d9822b"),
          ("javaid, 16 threads", "javaid-j16", "#b5471b")]

def draw(fg, bg, grid, path, cs):
    plt.rcParams.update({"font.family": "DejaVu Sans", "text.color": fg, "axes.labelcolor": fg,
                         "xtick.color": fg, "ytick.color": fg})
    fig, ax = plt.subplots(figsize=(8, 4), dpi=200)
    fig.patch.set_facecolor(bg); ax.set_facecolor(bg)
    w = 0.26
    for i, (label, key, color) in enumerate(series):
        xs = [g + (i - 1) * w for g in range(len(groups))]
        ys = [med[f"{g[1]}/{key}"] for g in groups]
        c = cs if key == "checkstyle" else color
        bars = ax.bar(xs, ys, w, label=label, color=c)
        for b, y in zip(bars, ys):
            ax.text(b.get_x() + b.get_width() / 2, y + 2, f"{y:.0f}" if y >= 10 else f"{y:.1f}",
                    ha="center", va="bottom", fontsize=8, color=fg)
    ax.set_xticks(range(len(groups))); ax.set_xticklabels([g[0] for g in groups])
    ax.set_ylabel("wall-clock seconds (lower is faster)")
    ax.set_ylim(0, 175)
    ax.yaxis.grid(True, color=grid); ax.set_axisbelow(True)
    for s in ("top", "right"): ax.spines[s].set_visible(False)
    for s in ("left", "bottom"): ax.spines[s].set_color(grid)
    ax.set_title("Full bundled configs, median of runs. i7-10700 desktop. Checkstyle is single-threaded.",
                 fontsize=9, color=fg, loc="left")
    leg = ax.legend(frameon=False, fontsize=9); [t.set_color(fg) for t in leg.get_texts()]
    fig.tight_layout(); fig.savefig(path, format="png", facecolor=bg); plt.close(fig)

out = os.path.join(HERE, "..", "docs", "img"); os.makedirs(out, exist_ok=True)
draw("#24292f", "#ffffff", "#d0d7de", os.path.join(out, "benchmark-light.png"), "#8b8b8b")
draw("#e6edf3", "#0d1117", "#30363d", os.path.join(out, "benchmark-dark.png"), "#8b949e")
