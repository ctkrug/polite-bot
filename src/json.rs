//! A minimal JSON string encoder shared by the modules that hand-roll their
//! `to_json` output. Keeping the wasm boundary primitive-only means no serde
//! dependency, but the string-escaping still has to be correct per RFC 8259 —
//! this is the single copy every module escapes user-controlled text through.

/// Encodes `s` as a JSON string literal, including the surrounding quotes,
/// escaping quotes, backslashes, the named control chars, and any remaining
/// control char below U+0020 as a `\u00xx` sequence.
pub(crate) fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn escapes_control_characters() {
        // json_string is the encoder every module escapes user-controlled
        // text through — keep it correct per RFC 8259 so a finding or a fix
        // that quotes pasted source back to the caller stays safe.
        assert_eq!(encode("a\tb\rc"), "\"a\\tb\\rc\"");
    }

    #[test]
    fn escapes_quotes_and_backslashes() {
        assert_eq!(encode(r#"he said "hi"\ "#), r#""he said \"hi\"\\ ""#);
    }

    #[test]
    fn escapes_low_control_chars_as_unicode() {
        assert_eq!(encode("\u{0007}"), "\"\\u0007\"");
    }
}
