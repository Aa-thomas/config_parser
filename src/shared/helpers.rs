/// Tiny utility to show a caret-line snippet; adjust as you like.
fn extract_snippet(src: &str, line: usize, column: usize) -> String {
    let mut out = String::new();
    if let Some(l) = src.lines().nth(line.saturating_sub(1)) {
        out.push_str(l);
        out.push('\n');
        if column > 1 {
            out.push_str(&" ".repeat(column.saturating_sub(1)));
        }
        out.push('^');
    }
    out
}

/// Pick the closest-looking string from `hay` for a given `needle`.
/// Scoring favors longer common prefixes and similar lengths; returns the best match or `None`.
/// Useful for friendly “did you mean …?” suggestions.
pub fn suggest<'a>(needle: &str, hay: impl IntoIterator<Item = &'a str>) -> Option<String> {
    let mut best: Option<(usize, &str)> = None;

    for h in hay {
        let len_diff = needle.len().abs_diff(h.len());
        let common_prefix = needle
            .chars()
            .zip(h.chars())
            .take_while(|(a, b)| a == b)
            .count();

        let score = len_diff.saturating_sub(common_prefix);

        if best.map_or(true, |(best_score, _)| score < best_score) {
            best = Some((score, h));
        }
    }

    best.map(|(_, h)| h.to_string())
}
