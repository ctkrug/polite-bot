import init, { version, score_scraper } from "./pkg/politebot_core.js";

const EXAMPLE = `import requests

for url in urls:
    requests.get(url)
`;

const sourceInput = document.getElementById("source-input");
const verdictStatus = document.getElementById("verdict-status");
const findingsList = document.getElementById("findings-list");
const engineStatus = document.getElementById("engine-status");

function renderScore(json) {
  const report = JSON.parse(json);

  verdictStatus.textContent = `verdict: ${report.verdict}`;
  verdictStatus.className = `verdict-status verdict-${report.verdict}`;

  findingsList.innerHTML = "";
  for (const finding of report.findings) {
    const item = document.createElement("li");
    const lineNo = document.createElement("span");
    lineNo.className = "line-no";
    lineNo.textContent = `L${finding.line}`;
    item.appendChild(lineNo);
    item.appendChild(document.createTextNode(finding.message));
    findingsList.appendChild(item);
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
