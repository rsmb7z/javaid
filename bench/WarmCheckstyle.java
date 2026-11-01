import com.puppycrawl.tools.checkstyle.Checker;
import com.puppycrawl.tools.checkstyle.ConfigurationLoader;
import com.puppycrawl.tools.checkstyle.PropertiesExpander;
import com.puppycrawl.tools.checkstyle.api.Configuration;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;

/**
 * "What if the JVM stays resident?" (IDE plugin, Gradle daemon, a lint server).
 * Lints the same file repeatedly inside ONE JVM and reports first-call and warm latency, plus the
 * process's peak resident memory. A measuring stick, not a product.
 *
 * usage: WarmCheckstyle <checkstyle-config.xml> <file-or-dir> <iterations>
 */
public final class WarmCheckstyle {
    public static void main(String[] args) throws Exception {
        String config = args[0];
        String input = args[1];
        int n = Integer.parseInt(args[2]);
        long[] ns = new long[n];

        Configuration conf = ConfigurationLoader.loadConfiguration(config, new PropertiesExpander(System.getProperties()));
        Checker checker = new Checker();
        checker.setModuleClassLoader(Checker.class.getClassLoader());
        checker.configure(conf);
        List<File> files = List.of(new File(input));
        for (int i = 0; i < n; i++) {
            long t0 = System.nanoTime();
            checker.process(files);
            ns[i] = System.nanoTime() - t0;
        }
        checker.destroy();

        long[] tail = Arrays.copyOfRange(ns, n / 2, n);
        Arrays.sort(tail);
        System.out.printf(
            "checkstyle iterations=%d first=%.1fms tenth=%.1fms warm_median=%.2fms warm_min=%.2fms warm_p95=%.2fms peak_rss=%dMB%n",
            n, ns[0] / 1e6, ns[Math.min(9, n - 1)] / 1e6,
            tail[tail.length / 2] / 1e6, tail[0] / 1e6, tail[(int) (tail.length * 0.95)] / 1e6, peakRssMb());
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
