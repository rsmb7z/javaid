//! Columns and line widths the way Checkstyle measures them, a tab running to the next multiple of
//! the tab width.
//!
//! Checkstyle measures in more than one unit, and the differences are visible in its output (all
//! probed), so they are kept apart here by name rather than folded into one function:
//!
//! * [`expanded_column`]: the column of a finding on the syntax tree. The position counts code
//!   points, the expansion walks that many UTF-16 *units* (`CommonUtil.lengthExpandedTabs`).
//! * [`expanded_chars`] / [`chars_width`]: a ported check's `log(line, columnNo)`, where the column
//!   is a code-point index and expansion walks code points.
//! * [`line_length`]: what `LineLength` reports, expanded over UTF-16 units and then reported in
//!   code points.
//!
//! Every one of them advances a tab with [`tab_stop`], as does `Indentation`, which measures the
//! UTF-16 lines of the Java `String`s it was ported from.

/// The width after a tab that starts at `width`.
#[inline]
pub fn tab_stop(width: usize, tab_width: usize) -> usize {
    (width / tab_width + 1) * tab_width
}

/// The column Checkstyle prints for the position after `before`, the bytes of the line up to that
/// position.
///
/// Checkstyle's parser counts code points, so the position is `n` characters in; the tab expansion
/// then walks the first `n` UTF-16 *units* of the line (`CommonUtil.lengthExpandedTabs`), a tab
/// running to the next multiple of `tab_width`. They are the same thing unless a supplementary
/// character (an emoji) comes first, when the walk stops short by one unit for each of them
/// (probed): a tab after an emoji expands from a column that is too small.
pub fn expanded_column(before: &[u8], tab_width: usize) -> u32 {
    if before.is_ascii() {
        let mut width = 0usize;
        for &b in before {
            width = if b == b'\t' { tab_stop(width, tab_width) } else { width + 1 };
        }
        return width as u32 + 1;
    }
    let text = String::from_utf8_lossy(before);
    let characters = text.chars().count();
    let mut width = 0usize;
    for unit in text.encode_utf16().take(characters) {
        width = if unit == u16::from(b'\t') { tab_stop(width, tab_width) } else { width + 1 };
    }
    width as u32 + 1
}

/// The tab-expanded width of the first `chars` code points of `line`, and how many code points were
/// there to measure (fewer than `chars` when the line ends first).
pub fn expanded_chars(line: &[u8], chars: usize, tab_width: usize) -> (usize, usize) {
    let mut width = 0usize;
    let mut seen = 0usize;
    let mut step = |c: bool| {
        width = if c { tab_stop(width, tab_width) } else { width + 1 };
        seen += 1;
    };
    if line.is_ascii() {
        line.iter().take(chars).for_each(|&b| step(b == b'\t'));
    } else {
        String::from_utf8_lossy(line).chars().take(chars).for_each(|c| step(c == '\t'));
    }
    (width, seen)
}

/// `CommonUtil.lengthExpandedTabs(line, chars, tabWidth)` as the ported checks' `log` uses it: the
/// width of the first `chars` code points, where a position past the end of the line counts one.
pub fn chars_width(line: &[u8], chars: usize, tab_width: usize) -> usize {
    let (width, seen) = expanded_chars(line, chars, tab_width);
    width + (chars - seen)
}

/// The length Checkstyle reports for a line: tabs run to the next multiple of `tab_width` counted in
/// UTF-16 units, where a supplementary character takes two, and then each such character is taken
/// off once (probed: `LineLength` expands over the string's units but reports in code points).
pub fn line_length(line: &[u8], tab_width: usize) -> usize {
    let (mut len, mut pairs) = (0, 0);
    for &b in line {
        if b == b'\t' {
            len = tab_stop(len, tab_width);
        } else if b & 0xC0 != 0x80 {
            len += 1; // not a UTF-8 continuation byte, so the start of a code point
            if b >= 0xF0 {
                len += 1;
                pairs += 1;
            }
        }
    }
    len - pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tab_runs_to_the_next_stop() {
        assert_eq!(expanded_column(b"", 8), 1);
        assert_eq!(expanded_column(b"\t", 8), 9);
        assert_eq!(expanded_column(b"ab\tc", 4), 6);
        assert_eq!(chars_width(b"ab\tc", 4, 4), 5);
        assert_eq!(line_length(b"ab\tc", 4), 5);
    }

    #[test]
    fn the_units_differ_after_a_supplementary_character() {
        let line = "\u{1F600}\tx".as_bytes();
        // Two code points before `x`: the UTF-16 walk sees the emoji's two units and stops before the tab.
        assert_eq!(expanded_column(&line[..5], 8), 3);
        // Code points: the emoji is one, then the tab runs to 8.
        assert_eq!(chars_width(line, 2, 8), 8);
        // Expanded over units (2 + tab to 8 + 1), then the pair taken off once.
        assert_eq!(line_length(line, 8), 8);
    }

    #[test]
    fn positions_past_the_end_count_one_each() {
        assert_eq!(expanded_chars(b"a\t", 5, 4), (4, 2));
        assert_eq!(chars_width(b"a\t", 5, 4), 7);
    }
}
