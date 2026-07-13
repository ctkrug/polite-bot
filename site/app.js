import init, { version, score_scraper, suggest_fix } from "./pkg/politebot_core.js";

const EXAMPLE = `import requests

for url in urls:
    requests.get(url)
`;

const sourceInput = document.getElementById("source-input");
const verdictStatus = document.getElementById("verdict-status");
const findingsList = document.getElementById("findings-list");
const engineStatus = document.getElementById("engine-status");
const findingTemplate = document.getElementById("finding-template");

function buildFindingItem(finding) {
  const item = findingTemplate.content.firstElementChild.cloneNode(true);
  item.querySelector(".line-no").textContent = `L${finding.line}`;
  item.querySelector(".finding-message").textContent = finding.message;

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

function renderScore(json) {
  const report = JSON.parse(json);

  verdictStatus.textContent = `verdict: ${report.verdict}`;
  verdictStatus.className = `verdict-status verdict-${report.verdict}`;

  findingsList.innerHTML = "";
  for (const finding of report.findings) {
    findingsList.appendChild(buildFindingItem(finding));
  }
}

let debounceHandle;
function scheduleScore() {
  clearTimeout(debounceHandle);
  debounceHandle = setTimeout(() => renderScore(score_scraper(sourceInput.value)), 150);
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
  sourceInput.value = EXAMPLE;
  sourceInput.addEventListener("input", scheduleScore);
  renderScore(score_scraper(sourceInput.value));
}

main();
