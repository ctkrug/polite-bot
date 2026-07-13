//! Static analysis of pasted scraper source for politeness signals: a
//! declared `User-Agent` and some form of rate limiting between requests.
//! This is the heuristic seed BUILD expands with framework-specific
//! detection (Story 2.1/2.2 in docs/BACKLOG.md).

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Green,
    Yellow,
    Red,
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Verdict::Green => "green",
            Verdict::Yellow => "yellow",
            Verdict::Red => "red",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct PolitenessScore {
    pub verdict: Verdict,
    pub findings: Vec<Finding>,
}

impl PolitenessScore {
    /// Hand-rolled JSON so the wasm boundary stays dependency-free; a
    /// serde_json migration is a drop-in if the report shape grows.
    pub fn to_json(&self) -> String {
        let findings = self
            .findings
            .iter()
            .map(|f| format!(r#"{{"line":{},"message":{}}}"#, f.line, json_string(&f.message)))
            .collect::<Vec<_>>()
            .join(",");
        format!(r#"{{"verdict":"{}","findings":[{}]}}"#, self.verdict, findings)
    }
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

const BACKOFF_SIGNALS: &[&str] = &["time.sleep", "asyncio.sleep", "sleep(", "setTimeout", "RateLimiter", "backoff"];
const USER_AGENT_SIGNALS: &[&str] = &["User-Agent", "user_agent", "user-agent"];

/// Scans pasted scraper source line by line for the two signals BUILD's
/// backlog expands on: a declared User-Agent, and some form of rate
/// limiting/backoff between requests. Never panics on arbitrary input —
/// worst case it finds nothing and reports Red.
pub fn analyze(source: &str) -> PolitenessScore {
    let has_user_agent = USER_AGENT_SIGNALS.iter().any(|kw| source.contains(kw));
    let backoff_line = source
        .lines()
        .enumerate()
        .find(|(_, line)| BACKOFF_SIGNALS.iter().any(|kw| line.contains(kw)));

    let mut findings = Vec::new();

    if !has_user_agent {
        findings.push(Finding {
            line: first_request_line(source),
            message: "no User-Agent header found — scrapers should identify themselves".into(),
        });
    }

    if backoff_line.is_none() {
        findings.push(Finding {
            line: first_request_line(source),
            message: "no rate limiting or backoff detected between requests".into(),
        });
    }

    let verdict = match findings.len() {
        0 => Verdict::Green,
        1 => Verdict::Yellow,
        _ => Verdict::Red,
    };

    PolitenessScore { verdict, findings }
}

/// Best-effort line to attach a finding to: the first line that looks like
/// an outbound request call, or line 1 if nothing matches.
fn first_request_line(source: &str) -> usize {
    const REQUEST_SIGNALS: &[&str] = &[".get(", ".post(", "fetch(", "urlopen(", "requests."];
    source
        .lines()
        .enumerate()
        .find(|(_, line)| REQUEST_SIGNALS.iter().any(|kw| line.contains(kw)))
        .map(|(i, _)| i + 1)
        .unwrap_or(1)
}
