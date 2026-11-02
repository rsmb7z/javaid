import com.puppycrawl.tools.checkstyle.Checker;
import com.puppycrawl.tools.checkstyle.ConfigurationLoader;
import com.puppycrawl.tools.checkstyle.PropertiesExpander;
import com.puppycrawl.tools.checkstyle.api.AuditEvent;
import com.puppycrawl.tools.checkstyle.api.AuditListener;
import com.puppycrawl.tools.checkstyle.api.Configuration;
import java.io.File;
import java.io.PrintStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

/**
 * Runs one Checkstyle configuration over many files, one file at a time per checker, and keeps going
 * when a file makes a check throw (the command line stops at the first such file). Output is the
 * command line's, `[WARN] file:line:col: message [Rule]`; a file that throws prints `CRASH file` on
 * stderr and contributes no findings. Usage: BatchCheck CONFIG.xml LISTFILE [THREADS].
 */
public final class BatchCheck {
    public static void main(String[] args) throws Exception {
        final Configuration config = ConfigurationLoader.loadConfiguration(
            args[0], new PropertiesExpander(System.getProperties()));
        final List<String> files = new ArrayList<>(Files.readAllLines(Path.of(args[1])));
        files.removeIf(String::isEmpty);
        final int threads = args.length > 2 ? Integer.parseInt(args[2]) : 4;
        final PrintStream out = new PrintStream(System.out, false, "UTF-8");
        final ExecutorService pool = Executors.newFixedThreadPool(threads);
        for (int t = 0; t < threads; t++) {
            final int slot = t;
            pool.submit(() -> {
                try {
                    final Checker checker = new Checker();
                    checker.setModuleClassLoader(Checker.class.getClassLoader());
                    checker.configure(config);
                    final StringBuilder buf = new StringBuilder();
                    checker.addListener(new AuditListener() {
                        public void auditStarted(AuditEvent e) { }
                        public void auditFinished(AuditEvent e) { }
                        public void fileStarted(AuditEvent e) { }
                        public void fileFinished(AuditEvent e) { }
                        public void addError(AuditEvent e) {
                            final String src = e.getSourceName();
                            String shortName = src.substring(src.lastIndexOf('.') + 1);
                            if (shortName.endsWith("Check")) {
                                shortName = shortName.substring(0, shortName.length() - 5);
                            }
                            buf.append("[WARN] ").append(e.getFileName()).append(':').append(e.getLine());
                            if (e.getColumn() > 0) {
                                buf.append(':').append(e.getColumn());
                            }
                            buf.append(": ").append(e.getMessage()).append(" [").append(shortName).append("]\n");
                        }
                        public void addException(AuditEvent e, Throwable t) { }
                    });
                    for (int i = slot; i < files.size(); i += threads) {
                        final String f = files.get(i);
                        buf.setLength(0);
                        try {
                            checker.process(Collections.singletonList(new File(f)));
                            synchronized (out) {
                                out.print(buf);
                            }
                        } catch (Throwable e) {
                            synchronized (out) {
                                System.err.println("CRASH " + f);
                            }
                        }
                    }
                    checker.destroy();
                } catch (Throwable e) {
                    e.printStackTrace();
                }
            });
        }
        pool.shutdown();
        pool.awaitTermination(1, TimeUnit.DAYS);
        out.flush();
    }
}
