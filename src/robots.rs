//! A from-scratch robots.txt parser: groups directives by `User-agent` and
//! answers `is_allowed` queries by longest-match precedence between the
//! `Allow` and `Disallow` rules in the matching group.

#[derive(Debug, Clone, Default)]
pub struct RuleGroup {
    pub agents: Vec<String>,
    pub disallow: Vec<String>,
    pub allow: Vec<String>,
    pub crawl_delay: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct RobotsRules {
    pub groups: Vec<RuleGroup>,
}

fn flush_group(current: &mut Option<RuleGroup>, groups: &mut Vec<RuleGroup>) {
    if let Some(group) = current.take() {
        if !group.agents.is_empty() {
            groups.push(group);
        }
    }
}

/// Parses robots.txt text into rule groups. Unknown directives and malformed
/// lines are skipped rather than treated as errors — an empty or garbled
/// file simply parses to no groups, which `is_allowed` treats as allow-all.
pub fn parse(text: &str) -> RobotsRules {
    let mut groups: Vec<RuleGroup> = Vec::new();
    let mut current: Option<RuleGroup> = None;

    for raw_line in text.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim().to_string();

        match key.as_str() {
            "user-agent" => {
                let extend_current = matches!(
                    &current,
                    Some(group)
                        if group.disallow.is_empty()
                            && group.allow.is_empty()
                            && group.crawl_delay.is_none()
                );
                if extend_current {
                    current
                        .as_mut()
                        .unwrap()
                        .agents
                        .push(value.to_ascii_lowercase());
                } else {
                    flush_group(&mut current, &mut groups);
                    current = Some(RuleGroup {
                        agents: vec![value.to_ascii_lowercase()],
                        ..Default::default()
                    });
                }
            }
            "disallow" => {
                if let Some(group) = current.as_mut() {
                    if !value.is_empty() {
                        group.disallow.push(value);
                    }
                }
            }
            "allow" => {
                if let Some(group) = current.as_mut() {
                    if !value.is_empty() {
                        group.allow.push(value);
                    }
                }
            }
            "crawl-delay" => {
                if let Some(group) = current.as_mut() {
                    group.crawl_delay = value.parse().ok();
                }
            }
            _ => {}
        }
    }
    flush_group(&mut current, &mut groups);

    RobotsRules { groups }
}

impl RobotsRules {
    /// True if `path` is allowed for `agent`, per the most specific matching
    /// rule (longest prefix wins) in the group for `agent`, falling back to
    /// the wildcard `*` group, falling back to allow-all if no group matches.
    pub fn is_allowed(&self, agent: &str, path: &str) -> bool {
        let agent = agent.to_ascii_lowercase();
        let group = self
            .groups
            .iter()
            .find(|g| g.agents.iter().any(|a| a == &agent))
            .or_else(|| {
                self.groups
                    .iter()
                    .find(|g| g.agents.iter().any(|a| a == "*"))
            });

        let Some(group) = group else {
            return true;
        };

        let best_disallow = group
            .disallow
            .iter()
            .filter(|p| path.starts_with(p.as_str()))
            .map(|p| p.len())
            .max();
        let best_allow = group
            .allow
            .iter()
            .filter(|p| path.starts_with(p.as_str()))
            .map(|p| p.len())
            .max();

        match (best_disallow, best_allow) {
            (Some(d), Some(a)) => a >= d,
            (Some(_), None) => false,
            _ => true,
        }
    }

    /// The `Crawl-delay` (seconds) for `agent`, if the matching group declares one.
    pub fn crawl_delay(&self, agent: &str) -> Option<f32> {
        let agent = agent.to_ascii_lowercase();
        self.groups
            .iter()
            .find(|g| g.agents.iter().any(|a| a == &agent))
            .or_else(|| {
                self.groups
                    .iter()
                    .find(|g| g.agents.iter().any(|a| a == "*"))
            })
            .and_then(|g| g.crawl_delay)
    }
}
