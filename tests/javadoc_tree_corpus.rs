//! Dumps the Javadoc tree of every comment in a file of records (separated by U+0001), for
//! comparison with Checkstyle's own dump: `JT_IN=records JT_OUT=dump cargo test --test
//! javadoc_tree_corpus -- --ignored`.

use javaid::rules::javadoc_tree;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
#[ignore]
fn dump_records() {
    let (Ok(input), Ok(output)) = (std::env::var("JT_IN"), std::env::var("JT_OUT")) else { return };
    let text = std::fs::read_to_string(input).unwrap();
    let cur = Arc::new(AtomicUsize::new(0));
    let c2 = cur.clone();
    std::thread::spawn(move || {
        let mut last = usize::MAX;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(15));
            let now = c2.load(Ordering::Relaxed);
            if now == last {
                eprintln!("STUCK at record {now}");
                std::process::exit(3);
            }
            last = now;
        }
    });
    let mut out = String::new();
    for (i, rec) in text.split('\u{1}').enumerate() {
        cur.store(i, Ordering::Relaxed);
        out.push('\u{1}');
        if rec.is_empty() {
            out.push_str("EMPTY\n");
            continue;
        }
        let tree = javadoc_tree::parse(rec, 1, 0);
        out.push_str(&tree.dump(1));
    }
    std::fs::write(output, out).unwrap();
}
