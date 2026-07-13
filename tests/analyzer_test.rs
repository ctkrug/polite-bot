use politebot_core::{analyze, Verdict};

#[test]
fn flags_missing_user_agent_and_rate_limit() {
    let src = "import requests\nfor url in urls:\n    requests.get(url)\n";
    let score = analyze(src);
    assert_eq!(score.verdict, Verdict::Red);
    assert_eq!(score.findings.len(), 2);
    assert_eq!(score.findings[0].line, 3);
}

#[test]
fn yellow_when_only_rate_limit_missing() {
    let src = "headers = {\"User-Agent\": \"my-bot/1.0\"}\nrequests.get(url, headers=headers)\n";
    let score = analyze(src);
    assert_eq!(score.verdict, Verdict::Yellow);
    assert_eq!(score.findings.len(), 1);
}

#[test]
fn green_when_polite() {
    let src = "import time, requests\nheaders = {\"User-Agent\": \"my-bot/1.0\"}\nfor url in urls:\n    requests.get(url, headers=headers)\n    time.sleep(1)\n";
    let score = analyze(src);
    assert_eq!(score.verdict, Verdict::Green);
    assert!(score.findings.is_empty());
}

#[test]
fn to_json_produces_well_formed_output() {
    let score = analyze("requests.get(url)");
    let json = score.to_json();
    assert!(json.starts_with(r#"{"verdict":"red","findings":["#));
    assert!(json.contains(r#""line":1"#));
}

#[test]
fn recognizes_tenacity_retry_decorator_as_backoff() {
    let src = "headers = {\"User-Agent\": \"my-bot/1.0\"}\n@retry(wait=wait_exponential())\ndef fetch():\n    requests.get(url, headers=headers)\n";
    let score = analyze(src);
    assert_eq!(score.verdict, Verdict::Green);
    assert!(score.findings.is_empty());
}

#[test]
fn never_panics_on_empty_or_binary_looking_input() {
    let _ = analyze("");
    let _ = analyze("\u{0}\u{1}not real code\u{7f}");
}
