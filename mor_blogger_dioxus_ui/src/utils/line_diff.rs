//! Minimal line-based unified diff for the asset editor's Diff view.

/// Diff `old` against `new`, line by line. Returns `(tag, line)` pairs where
/// tag is `' '` (unchanged), `'-'` (only in old), or `'+'` (only in new).
// ponytail: O(n*m) LCS table — fine for asset-sized files; swap in Myers if a
// huge file ever makes the Diff toggle lag.
pub fn line_diff(old: &str, new: &str) -> Vec<(char, String)> {
    let a: Vec<&str> = old.lines().collect();
    let b: Vec<&str> = new.lines().collect();
    let (n, m) = (a.len(), b.len());

    // lcs[i][j] = length of the LCS of a[i..] and b[j..].
    let mut lcs = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }

    let mut out = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if a[i] == b[j] {
            out.push((' ', a[i].to_string()));
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            out.push(('-', a[i].to_string()));
            i += 1;
        } else {
            out.push(('+', b[j].to_string()));
            j += 1;
        }
    }
    for line in &a[i..] {
        out.push(('-', line.to_string()));
    }
    for line in &b[j..] {
        out.push(('+', line.to_string()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::line_diff;

    #[test]
    fn diff_marks_changed_added_and_kept_lines() {
        let old = "a\nb\nc\n";
        let new = "a\nB\nc\nd\n";
        let got = line_diff(old, new);
        let expect = vec![
            (' ', "a".to_string()),
            ('-', "b".to_string()),
            ('+', "B".to_string()),
            (' ', "c".to_string()),
            ('+', "d".to_string()),
        ];
        assert_eq!(got, expect);

        // Identical inputs: all lines unchanged.
        assert!(line_diff(old, old).iter().all(|(t, _)| *t == ' '));
        // Empty old: everything is an addition.
        assert!(line_diff("", "x\ny").iter().all(|(t, _)| *t == '+'));
    }
}
