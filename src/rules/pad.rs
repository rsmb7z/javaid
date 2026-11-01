//! What lies between a token and its neighbour: nothing, blanks, or a line end.

/// The gap on one side of a token. Running into the start or end of the file counts as a line end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gap {
    None,
    Space,
    Newline,
}

pub fn is_blank(b: u8) -> bool {
    // Java's Character.isWhitespace for ASCII, line breaks apart: they end the line.
    matches!(b, b' ' | b'\t' | 0x0b | 0x0c | 0x1c..=0x1f)
}

fn classify(blank: usize, at_edge_or_break: bool) -> Gap {
    if at_edge_or_break {
        Gap::Newline
    } else if blank > 0 {
        Gap::Space
    } else {
        Gap::None
    }
}

/// The gap after the token that ends at byte `end`.
pub fn after(src: &[u8], end: usize) -> Gap {
    let mut i = end;
    while i < src.len() && is_blank(src[i]) {
        i += 1;
    }
    classify(i - end, i >= src.len() || matches!(src[i], b'\n' | b'\r'))
}

/// The gap before the token that starts at byte `start`.
pub fn before(src: &[u8], start: usize) -> Gap {
    let mut i = start;
    while i > 0 && is_blank(src[i - 1]) {
        i -= 1;
    }
    classify(start - i, i == 0 || matches!(src[i - 1], b'\n' | b'\r'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_sees_blank_line_end_and_nothing() {
        assert_eq!(after(b"( x", 1), Gap::Space);
        assert_eq!(after(b"(\n x", 1), Gap::Newline);
        assert_eq!(after(b"(x", 1), Gap::None);
        assert_eq!(after(b"(", 1), Gap::Newline);
        assert_eq!(after(b"( \t\r\n", 1), Gap::Newline);
    }

    #[test]
    fn before_sees_blank_line_end_and_nothing() {
        assert_eq!(before(b"x )", 2), Gap::Space);
        assert_eq!(before(b"x\n)", 2), Gap::Newline);
        assert_eq!(before(b"x)", 1), Gap::None);
        assert_eq!(before(b")", 0), Gap::Newline);
    }
}
