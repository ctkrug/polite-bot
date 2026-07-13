# Design direction

## 1. Aesthetic direction

**polite_bot is terminal-mono:** a dark, scanline-textured console. You paste
code into what looks like a real terminal pane; the verdict comes back like a
linter's output — a traffic-light status line, monospace findings with line
numbers, and a blinking cursor on the wordmark. The tool's whole premise is
"scan this code and tell me what's wrong," so the UI should feel like running
a CLI linter, not filling out a web form.

This is a deliberate departure from a generic dark-card SaaS look: no rounded
glassy panels, no soft pastel accents. Sharp corners, a green/amber/red
signal palette that *is* the product's actual verdict system (not decoration
layered on top), and monospace type throughout.

## 2. Tokens

| Token | Value | Use |
|---|---|---|
| `--bg` | `#0a0f0c` | page background (near-black, faint green tint) |
| `--surface-1` | `#101914` | editor pane, cards |
| `--surface-2` | `#182420` | raised elements (buttons, header) |
| `--text` | `#dff5e8` | primary text |
| `--text-muted` | `#7fa593` | secondary text, comments, line numbers |
| `--accent` | `#35e08a` | primary accent — links, focus rings, green verdict |
| `--accent-support` | `#3fd1c8` | secondary accent — active states, highlights |
| `--warn` | `#f5c451` | yellow verdict |
| `--danger` | `#ff5f56` | red verdict, errors |

- **Type pairing:** display = `JetBrains Mono` (700, wordmark + headings),
  UI/body = `IBM Plex Mono` (400/500, everything else). Both monospace —
  hierarchy comes from weight and size, not a serif/sans contrast — because
  the whole page reads as "code."
- System fallback stack: `"JetBrains Mono", "IBM Plex Mono", ui-monospace,
  "SFMono-Regular", Menlo, Consolas, monospace`.
- **Spacing unit:** 8px scale (4px half-step allowed for tight inline gaps).
- **Corner radius:** 4px. Sharp, not rounded — a terminal, not a bubble.
- **Shadow/glow:** no drop shadows. Depth comes from a soft `--accent` glow
  (`box-shadow: 0 0 24px rgba(53, 224, 138, 0.15)`) on the active/focused
  panel, plus a 1px `--surface-2` border on raised elements.
- **Motion:** UI transitions 150ms ease-out; verdict/finding reveal 90ms
  ease-out (fast — it should feel like a scan completing, not a fade-in).

## 3. Layout intent

The hero **is** the paste-and-score flow: a terminal-styled code pane on the
left, a verdict panel (status line + findings list) on the right. Nothing
else competes with it above the fold.

- **1440×900 desktop:** two-column split, code pane ~60% width / verdict
  panel ~40%, both filling the viewport height below a slim header (no big
  marketing hero above it — the tool opens directly into the workspace).
- **390×844 phone:** stacked, code pane first (full width, ~50vh), verdict
  panel below it, sticky verdict summary bar pinned above the fold so the
  score is visible without scrolling.
- Header stays minimal: wordmark + a one-line tagline + a GitHub link. No nav
  bar bloat.

## 4. Signature detail

The wordmark `polite_bot` renders with a blinking block cursor
(`polite_bot█`) that never stops blinking — a live terminal, not a static
logo. The page background carries a very faint repeating scanline texture
(a `repeating-linear-gradient` at ~2% opacity) reinforcing the CRT-terminal
read without hurting text contrast.

## 5. Games/toys juice plan

Not applicable — polite_bot is a dev tool, not a game or playful toy. The
equivalent "feel good to use" bar is: the verdict panel updates within
300ms of a paste with a fast, snappy reveal (see Motion above), and each
finding gets a colored gutter marker in the code pane rather than a static
list disconnected from the source.
