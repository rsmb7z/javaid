import com.puppycrawl.tools.checkstyle.DetailNodeTreeStringPrinter;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;

public class Dump {
    public static void main(String[] a) throws Exception {
        String in = new String(Files.readAllBytes(Paths.get(a[0])), StandardCharsets.UTF_8);
        File tmp = File.createTempFile("jtd", ".txt");
        StringBuilder out = new StringBuilder();
        for (String rec : in.split("\u0001", -1)) {
            if (rec.isEmpty()) { out.append("\u0001EMPTY\n"); continue; }
            Files.write(tmp.toPath(), rec.getBytes(StandardCharsets.UTF_8));
            out.append('\u0001');
            try {
                out.append(DetailNodeTreeStringPrinter.printFileAst(tmp));
            } catch (Throwable t) {
                out.append("ERR ").append(String.valueOf(t.getMessage()).split("\n")[0]).append('\n');
            }
        }
        Files.write(Paths.get(a[1]), out.toString().getBytes(StandardCharsets.UTF_8));
    }
}
