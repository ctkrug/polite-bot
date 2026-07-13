//! Generates the one-click "add recommended User-Agent header" fix (Story
//! 1.2 in docs/BACKLOG.md) for the missing-User-Agent finding. Supports the
//! request call shapes the analyzer already recognizes; unsupported shapes
//! return `None` rather than guessing at a broken patch.

/// A suggested fix for a single finding: a unified-diff-style snippet and
/// the fully patched source, ready to copy back into the editor.
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub diff: String,
    pub patched_source: String,
}

impl FixSuggestion {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"diff":{},"patched_source":{}}}"#,
            json_string(&self.diff),
            json_string(&self.patched_source)
        )
    }
}

const DEFAULT_USER_AGENT: &str = "polite-bot/1.0 (+https://github.com/ctkrug/polite-bot)";

/// Suggests a fix for the missing-User-Agent finding at `line` (1-based) in
/// `source`. Returns `None` if `line` is out of range or the call on that
/// line doesn't match a request shape this fixer knows how to patch.
pub fn suggest_user_agent_fix(source: &str, line: usize) -> Option<FixSuggestion> {
    let lines: Vec<&str> = source.lines().collect();
    let idx = line.checked_sub(1)?;
    let original = *lines.get(idx)?;

    let patched_line = patch_python_requests_call(original)?;
    let indent = leading_whitespace(original);
    let header_line = format!(
        "{indent}headers = {{\"User-Agent\": \"{DEFAULT_USER_AGENT}\"}}"
    );

    let mut patched_lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    patched_lines[idx] = patched_line.clone();
    patched_lines.insert(idx, header_line.clone());

    let patched_source = patched_lines.join("\n");
    let diff = format!(
        "@@ -{line} +{line},{next} @@\n+{header_line}\n-{original}\n+{patched_line}",
        next = line + 1
    );

    Some(FixSuggestion {
        diff,
        patched_source,
    })
}

fn leading_whitespace(line: &str) -> String {
    line.chars().take_while(|c| c.is_whitespace()).collect()
}

/// Inserts `headers=headers` into a Python `requests.get(...)`-style call by
/// scanning for the call's matching closing paren (bracket-depth aware, so
/// nested calls like `requests.get(build_url(x))` still patch correctly).
fn patch_python_requests_call(line: &str) -> Option<String> {
    let open = find_call_open_paren(line, &["requests.get(", "requests.post(", ".get(", ".post("])?;
    let close = find_matching_close_paren(line, open)?;

    let args = line[open + 1..close].trim();
    let insertion = if args.is_empty() {
        "headers=headers".to_string()
    } else {
        format!("{args}, headers=headers")
    };

    Some(format!("{}{}{}", &line[..open + 1], insertion, &line[close..]))
}

fn find_call_open_paren(line: &str, markers: &[&str]) -> Option<usize> {
    markers
        .iter()
        .filter_map(|m| line.find(m).map(|i| i + m.len() - 1))
        .min()
}

fn find_matching_close_paren(line: &str, open: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut depth = 0i32;
    for (i, &b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
