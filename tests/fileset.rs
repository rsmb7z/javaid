//! The modules that read a set of files, not one: `JavadocPackage` and `Translation`. The expected
//! findings are what Checkstyle 14.4.0 printed for the same directories (recorded by hand: a fileset
//! is not a single probe file, so the golden-file harness cannot hold them).

use std::path::{Path, PathBuf};
use std::process::Command;

fn run(config: &str, dir: &Path) -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out = Command::new(env!("CARGO_BIN_EXE_javaid")).arg("--config").arg(root.join("bench/oracle").join(config)).arg(dir).output().expect("javaid runs");
    let prefix = format!("{}/", dir.parent().unwrap().display());
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout).lines().map(|l| l.replace(&prefix, "")).collect();
    lines.sort();
    lines
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("javaid-fileset-{}", std::process::id())).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn put(dir: &Path, name: &str, text: &str) {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

#[test]
fn javadoc_package_wants_a_package_info_in_each_directory() {
    let dir = scratch("pk");
    put(&dir, "sub/A.java", "class A {}\n");
    put(&dir, "legacy/C.java", "class C {}\n");
    put(&dir, "legacy/package.html", "<html/>");
    put(&dir, "ok/E.java", "class E {}\n");
    put(&dir, "ok/package-info.java", "package ok;\n");
    put(&dir, "module-info.java", "module m {}\n");
    let name = dir.file_name().unwrap().to_str().unwrap().to_string();
    let want = |extra: &[&str]| -> Vec<String> {
        let mut v: Vec<String> = extra.iter().map(|e| format!("{name}/{e}")).collect();
        v.sort();
        v
    };
    let tail = |lines: Vec<String>| -> Vec<String> { lines.into_iter().map(|l| l.splitn(2, '/').nth(1).map(|r| format!("{name}/{r}")).unwrap_or(l)).collect() };
    assert_eq!(tail(run("jp-default.xml", &dir)), want(&["legacy/C.java:1: JavadocPackage: Missing package-info.java file.", "sub/A.java:1: JavadocPackage: Missing package-info.java file."]));
    // With `allowLegacy` a package.html does instead of the package-info.java.
    assert_eq!(tail(run("jp-legacy.xml", &dir)), want(&["sub/A.java:1: JavadocPackage: Missing package-info.java file."]));
}

#[test]
fn translation_compares_the_keys_of_a_bundle() {
    let dir = scratch("tr");
    put(&dir, "messages.properties", "a=1\nb=2\nc=3\n");
    put(&dir, "messages_de.properties", "a=eins\nb=zwei\n");
    put(&dir, "messages_fr_CA.properties", "a=un\nz=extra\n");
    put(&dir, "other.properties", "x=1\n");
    put(&dir, "messages_es.properties", "only=1\n");
    let name = dir.file_name().unwrap().to_str().unwrap().to_string();
    let got: Vec<String> = run("tr-default.xml", &dir).into_iter().map(|l| l.splitn(2, '/').nth(1).map(|r| format!("{name}/{r}")).unwrap_or(l)).collect();
    let mut want: Vec<String> = [
        "messages.properties:1: Translation: Key 'only' missing.",
        "messages.properties:1: Translation: Key 'z' missing.",
        "messages_de.properties:1: Translation: Key 'c' missing.",
        "messages_de.properties:1: Translation: Key 'only' missing.",
        "messages_de.properties:1: Translation: Key 'z' missing.",
        "messages_es.properties:1: Translation: Key 'a' missing.",
        "messages_es.properties:1: Translation: Key 'b' missing.",
        "messages_es.properties:1: Translation: Key 'c' missing.",
        "messages_es.properties:1: Translation: Key 'z' missing.",
        "messages_fr_CA.properties:1: Translation: Key 'b' missing.",
        "messages_fr_CA.properties:1: Translation: Key 'c' missing.",
        "messages_fr_CA.properties:1: Translation: Key 'only' missing.",
    ]
    .iter()
    .map(|l| format!("{name}/{l}"))
    .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn translation_reads_keys_the_way_java_does() {
    let dir = scratch("tr2");
    put(&dir, "messages_de.properties", "k=1\n");
    put(&dir, "messages_fr.properties", "k=2\nk2=3\n");
    put(&dir, "messages.properties", "a\\ b=1\nc\\nd:2\n# comment\nu\\u0041=3\nlong=x\\\n  y\n");
    let name = dir.file_name().unwrap().to_str().unwrap().to_string();
    let got: Vec<String> = run("tr-default.xml", &dir).into_iter().map(|l| l.splitn(2, '/').nth(1).map(|r| format!("{name}/{r}")).unwrap_or(l)).collect();
    let mut want: Vec<String> = [
        "messages.properties:1: Translation: Key 'k' missing.",
        "messages.properties:1: Translation: Key 'k2' missing.",
        "messages_de.properties:1: Translation: Key 'a b' missing.",
        "messages_de.properties:1: Translation: Key 'c\\nd' missing.",
        "messages_de.properties:1: Translation: Key 'k2' missing.",
        "messages_de.properties:1: Translation: Key 'long' missing.",
        "messages_de.properties:1: Translation: Key 'uA' missing.",
        "messages_fr.properties:1: Translation: Key 'a b' missing.",
        "messages_fr.properties:1: Translation: Key 'c\\nd' missing.",
        "messages_fr.properties:1: Translation: Key 'long' missing.",
        "messages_fr.properties:1: Translation: Key 'uA' missing.",
    ]
    .iter()
    .map(|l| format!("{name}/{l}"))
    .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn translation_notices_a_missing_default_file() {
    let dir = scratch("tr3");
    put(&dir, "messages_de.properties", "k=1\n");
    let name = dir.file_name().unwrap().to_str().unwrap().to_string();
    let got: Vec<String> = run("tr-default.xml", &dir).into_iter().map(|l| l.splitn(2, '/').nth(1).map(|r| format!("{name}/{r}")).unwrap_or(l)).collect();
    assert_eq!(got, vec![format!("{name}:1: Translation: Properties file 'messages.properties' is missing.")]);
}
