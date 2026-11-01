import net.sourceforge.pmd.PMDConfiguration;
import net.sourceforge.pmd.PmdAnalysis;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

/**
 * PMD counterpart of WarmCheckstyle: the same file linted repeatedly inside ONE JVM. The ruleset is
 * re-loaded on every iteration (that is how the PmdAnalysis API is used), so this slightly
 * overstates what a purpose-built resident PMD server would pay.
 *
 * usage: WarmPmd <ruleset.xml> <file-or-dir> <iterations>
 */
public final class WarmPmd {
    public static void main(String[] args) throws Exception {
        String ruleset = args[0];
        String input = args[1];
        int n = Integer.parseInt(args[2]);
        long[] ns = new long[n];
        int violations = 0;

        for (int i = 0; i < n; i++) {
            long t0 = System.nanoTime();
            PMDConfiguration c = new PMDConfiguration();
            c.addInputPath(Path.of(input));
            c.addRuleSet(ruleset);
            c.setThreads(1);
            c.setIgnoreIncrementalAnalysis(true);
            try (PmdAnalysis a = PmdAnalysis.create(c)) {
                violations = a.performAnalysisAndCollectReport().getViolations().size();
            }
            ns[i] = System.nanoTime() - t0;
        }

        long[] tail = Arrays.copyOfRange(ns, n / 2, n);
        Arrays.sort(tail);
        System.out.printf(
            "pmd iterations=%d first=%.1fms tenth=%.1fms warm_median=%.2fms warm_min=%.2fms warm_p95=%.2fms peak_rss=%dMB violations_last=%d%n",
            n, ns[0] / 1e6, ns[Math.min(9, n - 1)] / 1e6,
            tail[tail.length / 2] / 1e6, tail[0] / 1e6, tail[(int) (tail.length * 0.95)] / 1e6, peakRssMb(), violations);
    }

    private static long peakRssMb() throws Exception {
        for (String line : Files.readAllLines(Path.of("/proc/self/status"))) {
            if (line.startsWith("VmHWM:")) {
                return Long.parseLong(line.replaceAll("[^0-9]", "")) / 1024;
            }
        }
        return -1;
    }
}
