import init, { version, score_scraper, suggest_fix, check_robots } from "./pkg/politebot_core.js";

const EXAMPLES = {
  worst: `import requests

for url in urls:
    requests.get(url)
`,
  rateLimited: `import time, requests

for url in urls:
    requests.get(url)
    time.sleep(1)
`,
  polite: `import time, requests

headers = {"User-Agent": "my-research-bot/1.0 (+https://example.com/bot)"}
for url in urls:
    requests.get(url, headers=headers)
    time.sleep(1)
`,
};

const sourceInput = document.getElementById("source-input");
const verdictStatus = document.getElementById("verdict-status");
const findingsList = document.getElementById("findings-list");
const engineStatus = document.getElementById("engine-status");
const findingTemplate = document.getElementById("finding-template");
const codeGutter = document.getElementById("code-gutter");

function renderGutter(source, findings, verdict) {
  const lineCount = Math.max(source.split("\n").length, 1);
  const flaggedLines = new Set(findings.map((f) => f.line));
  const markerClass = verdict === "red" ? "gutter-marker-danger" : "gutter-marker-warn";

  const frag = document.createDocumentFragment();
  for (let i = 1; i <= lineCount; i++) {
    const lineEl = document.createElement("div");
    lineEl.className = "gutter-line";
    if (flaggedLines.has(i)) {
      lineEl.classList.add(markerClass);
    }
    lineEl.textContent = String(i);
    frag.appendChild(lineEl);
  }

  codeGutter.replaceChildren(frag);
}

const RULE_EXPLANATIONS = {
  missing_user_agent:
    "Sites use the User-Agent header to tell real clients apart from anonymous bots. " +
    "Requests with no User-Agent are the first thing rate limiters and abuse filters " +
    "flag, and they're the hardest to trace back to you if a site operator wants to " +
    "reach out before blocking your IP outright.",
  default_user_agent:
    "A default User-Agent (the one your library sends automatically, like " +
    "python-requests/2.x) is easy for a site to bulk-block since thousands of other " +
    "scrapers send the exact same string. It also gives the target no way to identify " +
    "or contact you specifically if your traffic causes a problem.",
  missing_rate_limit:
    "Hammering a site with back-to-back requests is the single fastest way to get " +
    "IP-banned, trip a WAF, or — at real scale — generate an abuse complaint against " +
    "your hosting provider. A small delay or backoff between requests is basic " +
    "scraping courtesy and costs you almost nothing in wall-clock time.",
  unrecognized_source:
    "This source doesn't match a request-call pattern polite_bot recognizes (Python " +
    "requests/urllib, or JS fetch), so it can't confidently point at a specific line. " +
    "Double-check manually that outbound requests set a real User-Agent and are " +
    "throttled.",
};

function buildFindingItem(finding) {
  const item = findingTemplate.content.firstElementChild.cloneNode(true);
  item.querySelector(".line-no").textContent = `L${finding.line}`;
  item.querySelector(".finding-message").textContent = finding.message;

  const whyBtn = item.querySelector(".why-btn");
  const whyPanel = item.querySelector(".why-panel");
  item.querySelector(".why-text").textContent =
    RULE_EXPLANATIONS[finding.rule_id] || "No further explanation available for this rule.";
  whyBtn.addEventListener("click", () => {
    whyPanel.hidden = !whyPanel.hidden;
  });

  const fixJson = suggest_fix(sourceInput.value, finding.line);
  if (fixJson) {
    wireFixButton(item, JSON.parse(fixJson));
  }

  return item;
}

function wireFixButton(item, fix) {
  const fixBtn = item.querySelector(".fix-btn");
  const fixPanel = item.querySelector(".fix-panel");
  const diffEl = item.querySelector(".fix-diff");
  const copyBtn = item.querySelector(".copy-btn");
  const copyStatus = item.querySelector(".copy-status");

  fixBtn.hidden = false;
  diffEl.textContent = fix.diff;

  fixBtn.addEventListener("click", () => {
    fixPanel.hidden = !fixPanel.hidden;
  });

  copyBtn.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(fix.patched_source);
      copyStatus.textContent = "copied!";
    } catch (err) {
      copyStatus.textContent = "copy failed — select and copy manually";
      console.error(err);
    }
  });
}

let lastReport = null;

function buildMarkdownReport(report) {
  const lines = [`## polite_bot verdict: ${report.verdict.toUpperCase()}`, ""];
  if (report.findings.length === 0) {
    lines.push("No findings — this scraper looks polite.");
  } else {
    for (const finding of report.findings) {
      lines.push(`- **L${finding.line}** — ${finding.message}`);
    }
  }
  lines.push("", "_scored with polite_bot — https://github.com/ctkrug/polite-bot_");
  return lines.join("\n");
}

function renderScore(json) {
  const report = JSON.parse(json);
  lastReport = report;

  verdictStatus.textContent = `verdict: ${report.verdict}`;
  verdictStatus.className = `verdict-status verdict-${report.verdict}`;

  findingsList.innerHTML = "";
  for (const finding of report.findings) {
    findingsList.appendChild(buildFindingItem(finding));
  }

  renderGutter(sourceInput.value, report.findings, report.verdict);
}

let debounceHandle;
function scheduleScore() {
  clearTimeout(debounceHandle);
  debounceHandle = setTimeout(() => renderScore(score_scraper(sourceInput.value)), 150);
}

const robotsAgentInput = document.getElementById("robots-agent");
const robotsPathInput = document.getElementById("robots-path");
const robotsTxtInput = document.getElementById("robots-txt");
const robotsResult = document.getElementById("robots-result");

function renderRobotsCheck() {
  const path = robotsPathInput.value.trim();
  if (!path) {
    robotsResult.textContent = "enter a target path to check";
    robotsResult.className = "robots-result";
    return;
  }

  const agent = robotsAgentInput.value.trim() || "*";
  const allowed = check_robots(robotsTxtInput.value, agent, path);

  robotsResult.textContent = allowed
    ? `allowed: ${agent} may request ${path}`
    : `disallowed: robots.txt blocks ${agent} from ${path}`;
  robotsResult.className = `robots-result robots-result-${allowed ? "allow" : "deny"}`;
}

let robotsDebounceHandle;
function scheduleRobotsCheck() {
  clearTimeout(robotsDebounceHandle);
  robotsDebounceHandle = setTimeout(renderRobotsCheck, 150);
}

async function main() {
  try {
    await init();
  } catch (err) {
    engineStatus.textContent = "engine: failed to load";
    verdictStatus.textContent = "wasm engine failed to load — see console";
    verdictStatus.className = "verdict-status verdict-red";
    console.error(err);
    return;
  }

  engineStatus.textContent = `engine: politebot-core v${version()}`;
  sourceInput.value = EXAMPLES.worst;
  sourceInput.addEventListener("input", scheduleScore);
  sourceInput.addEventListener("scroll", () => {
    codeGutter.scrollTop = sourceInput.scrollTop;
  });
  for (const btn of document.querySelectorAll(".example-btn")) {
    btn.addEventListener("click", () => {
      const example = EXAMPLES[btn.dataset.example];
      if (!example) return;
      clearTimeout(debounceHandle);
      sourceInput.value = example;
      renderScore(score_scraper(sourceInput.value));
    });
  }
  renderScore(score_scraper(sourceInput.value));

  robotsAgentInput.addEventListener("input", scheduleRobotsCheck);
  robotsPathInput.addEventListener("input", scheduleRobotsCheck);
  robotsTxtInput.addEventListener("input", scheduleRobotsCheck);

  const copyReportBtn = document.getElementById("copy-report-btn");
  const copyReportStatus = document.getElementById("copy-report-status");
  copyReportBtn.addEventListener("click", async () => {
    if (!lastReport) return;
    try {
      await navigator.clipboard.writeText(buildMarkdownReport(lastReport));
      copyReportStatus.textContent = "copied!";
    } catch (err) {
      copyReportStatus.textContent = "copy failed — select and copy manually";
      console.error(err);
    }
  });
}

main();
