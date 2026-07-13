use politebot_core::suggest_user_agent_fix;

#[test]
fn patches_simple_requests_get_call() {
    let src = "import requests\nfor url in urls:\n    requests.get(url)\n";
    let fix = suggest_user_agent_fix(src, 3).expect("expected a fix for requests.get");

    assert!(fix.patched_source.contains("headers = {\"User-Agent\":"));
    assert!(fix
        .patched_source
        .contains("requests.get(url, headers=headers)"));
    assert!(fix.diff.contains("-    requests.get(url)"));
    assert!(fix.diff.contains("+    requests.get(url, headers=headers)"));
}

#[test]
fn preserves_indentation_of_the_original_line() {
    let src = "def scrape():\n        requests.get(url)\n";
    let fix = suggest_user_agent_fix(src, 2).unwrap();
    assert!(fix.patched_source.contains("        headers = {"));
    assert!(fix
        .patched_source
        .contains("        requests.get(url, headers=headers)"));
}

#[test]
fn handles_nested_calls_in_the_argument_list() {
    let src = "requests.get(build_url(base, path))\n";
    let fix = suggest_user_agent_fix(src, 1).unwrap();
    assert!(fix
        .patched_source
        .contains("requests.get(build_url(base, path), headers=headers)"));
}

#[test]
fn handles_call_with_no_arguments() {
    let src = "requests.get()\n";
    let fix = suggest_user_agent_fix(src, 1).unwrap();
    assert!(fix.patched_source.contains("requests.get(headers=headers)"));
}

#[test]
fn patches_simple_fetch_call() {
    let src = "async function load() {\n  const res = await fetch(url);\n}\n";
    let fix = suggest_user_agent_fix(src, 2).unwrap();
    assert!(fix
        .patched_source
        .contains("fetch(url, { headers: { \"User-Agent\": "));
    assert!(fix.diff.contains("-  const res = await fetch(url);"));
}

#[test]
fn declines_fetch_call_with_existing_options_argument() {
    let src = "fetch(url, { method: \"GET\" })\n";
    assert!(suggest_user_agent_fix(src, 1).is_none());
}

#[test]
fn declines_fetch_call_with_no_arguments() {
    assert!(suggest_user_agent_fix("fetch()\n", 1).is_none());
}

#[test]
fn returns_none_for_unsupported_call_shape() {
    assert!(suggest_user_agent_fix("urlopen(url)\n", 1).is_none());
}

#[test]
fn returns_none_for_out_of_range_line() {
    let src = "requests.get(url)\n";
    assert!(suggest_user_agent_fix(src, 0).is_none());
    assert!(suggest_user_agent_fix(src, 99).is_none());
}

#[test]
fn to_json_produces_well_formed_output() {
    let fix = suggest_user_agent_fix("requests.get(url)\n", 1).unwrap();
    let json = fix.to_json();
    assert!(json.starts_with(r#"{"diff":"#));
    assert!(json.contains(r#""patched_source":"#));
    assert!(json.contains("headers=headers"));
}
