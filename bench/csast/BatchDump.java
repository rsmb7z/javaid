import com.puppycrawl.tools.checkstyle.AstTreeStringPrinter;
import com.puppycrawl.tools.checkstyle.JavaParser;
import java.io.File;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

/**
 * Runs Checkstyle's own AST printer over many files in one JVM: the oracle for csast.
 * Usage: BatchDump (t|T) LISTFILE OUTDIR [THREADS]. Each path in LISTFILE gets OUTDIR/PATH.txt, where PATH
 * is the path without its leading slash and with '/' replaced by '%', holding the `-t` (or `-T`) dump, or
 * OUTDIR/PATH.err when Checkstyle fails to parse the file.
 */
public final class BatchDump {
    public static void main(String[] args) throws Exception {
        final JavaParser.Options options =
            args[0].equals("T") ? JavaParser.Options.WITH_COMMENTS : JavaParser.Options.WITHOUT_COMMENTS;
        final List<String> files = Files.readAllLines(Path.of(args[1]));
        final Path out = Path.of(args[2]);
        Files.createDirectories(out);
        final int threads = args.length > 3 ? Integer.parseInt(args[3]) : 4;
        final ExecutorService pool = Executors.newFixedThreadPool(threads);
        for (int idx = 0; idx < files.size(); idx++) {
            final String f = files.get(idx);
            final int fidx = idx;
            if (f.isEmpty()) {
                continue;
            }
            pool.submit(() -> {
                final String name = String.format("%06d", fidx);
                try {
                    final String dump = AstTreeStringPrinter.printFileAst(new File(f), options);
                    Files.writeString(out.resolve(name + ".txt"), dump, StandardCharsets.UTF_8);
                } catch (Throwable e) {
                    try {
                        Throwable root = e; while (root.getCause() != null) { root = root.getCause(); } Files.writeString(out.resolve(name + ".err"), String.valueOf(root), StandardCharsets.UTF_8);
                    } catch (Exception ignored) {
                        // nothing to do
                    }
                }
            });
        }
        pool.shutdown();
        pool.awaitTermination(1, TimeUnit.DAYS);
    }
}
