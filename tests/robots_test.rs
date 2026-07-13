use politebot_core::parse_robots;

#[test]
fn disallow_blocks_matching_prefix() {
    let robots = parse_robots("User-agent: *\nDisallow: /private\n");
    assert!(!robots.is_allowed("PoliteBot", "/private/data"));
    assert!(robots.is_allowed("PoliteBot", "/public"));
}

#[test]
fn allow_overrides_more_specific_disallow() {
    let robots = parse_robots("User-agent: *\nDisallow: /assets\nAllow: /assets/public\n");
    assert!(robots.is_allowed("PoliteBot", "/assets/public/logo.png"));
    assert!(!robots.is_allowed("PoliteBot", "/assets/private/logo.png"));
}

#[test]
fn equal_length_allow_and_disallow_favors_allow() {
    // When an Allow and a Disallow rule match with equal specificity
    // (same path length), the least-restrictive rule wins per robots.txt
    // convention — ties must not silently fall through to blocking.
    let robots = parse_robots("User-agent: *\nDisallow: /path\nAllow: /path\n");
    assert!(robots.is_allowed("AnyBot", "/path"));
}

#[test]
fn grouped_user_agents_share_rules() {
    let robots = parse_robots("User-agent: BotA\nUser-agent: BotB\nDisallow: /no-bots\n");
    assert!(!robots.is_allowed("BotA", "/no-bots/page"));
    assert!(!robots.is_allowed("BotB", "/no-bots/page"));
}

#[test]
fn specific_agent_group_beats_wildcard() {
    let robots =
        parse_robots("User-agent: *\nDisallow: /\n\nUser-agent: PoliteBot\nDisallow:\nAllow: /\n");
    assert!(robots.is_allowed("PoliteBot", "/anything"));
    assert!(!robots.is_allowed("OtherBot", "/anything"));
}

#[test]
fn crawl_delay_is_parsed_per_group() {
    let robots = parse_robots("User-agent: *\nCrawl-delay: 2.5\n");
    assert_eq!(robots.crawl_delay("PoliteBot"), Some(2.5));
}

#[test]
fn missing_robots_txt_allows_everything() {
    let robots = parse_robots("");
    assert!(robots.is_allowed("AnyBot", "/whatever"));
    assert_eq!(robots.crawl_delay("AnyBot"), None);
}

#[test]
fn comments_and_malformed_lines_are_ignored() {
    let robots = parse_robots(
        "# a comment\nnot a directive\nUser-agent: *\nDisallow: /x # trailing comment\n",
    );
    assert!(!robots.is_allowed("AnyBot", "/x/y"));
}
