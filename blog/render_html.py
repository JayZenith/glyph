#!/usr/bin/env python3
"""Render finalized_blogv3.md to a standalone, readable HTML article."""
from __future__ import annotations

from pathlib import Path

import markdown


ROOT = Path(__file__).resolve().parent
SOURCE = ROOT / "finalized_blogv3.md"
TARGET = ROOT / "index.html"


CSS = r"""
:root {
  --bg: #f2f4f7;
  --paper: #ffffff;
  --ink: #17191d;
  --muted: #626b77;
  --line: #d9dee7;
  --soft: #f7f8fa;
  --code-bg: #101820;
  --code-ink: #f4f7fb;
  --link: #1f5fbf;
  --accent: #2b6f74;
}
* { box-sizing: border-box; }
html { scroll-behavior: smooth; }
body {
  margin: 0;
  background: var(--bg);
  color: var(--ink);
  font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  line-height: 1.68;
}
a {
  color: var(--link);
  text-decoration-thickness: 1px;
  text-underline-offset: 3px;
}
.shell {
  max-width: 1060px;
  margin: 0 auto;
  background: var(--paper);
  box-shadow: 0 0 0 1px rgba(17, 24, 39, 0.06), 0 28px 80px rgba(17, 24, 39, 0.10);
}
.top {
  padding: 56px 72px 28px;
  border-bottom: 1px solid var(--line);
  background: linear-gradient(180deg, #ffffff 0%, #fafbfc 100%);
}
.kicker {
  margin: 0 0 14px;
  color: var(--accent);
  font-size: 13px;
  font-weight: 800;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}
.top h1 {
  max-width: 880px;
  margin: 0;
  font-size: clamp(38px, 5.8vw, 64px);
  line-height: 1.02;
  letter-spacing: 0;
}
.lede {
  max-width: 780px;
  margin: 22px 0 0;
  color: #39414d;
  font-size: 19px;
  line-height: 1.55;
}
.article {
  padding: 38px 72px 76px;
}
.article > h1 { display: none; }
.article h2 {
  max-width: 820px;
  margin: 58px 0 18px;
  padding-top: 8px;
  border-top: 1px solid var(--line);
  font-size: 28px;
  line-height: 1.18;
  letter-spacing: 0;
}
.article h2:first-of-type {
  margin-top: 24px;
}
.article p,
.article ul,
.article ol {
  max-width: 780px;
  font-size: 17px;
}
.article p {
  margin: 17px 0;
}
.article strong {
  font-weight: 780;
}
.article code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
  font-size: 0.91em;
  background: #eef1f5;
  border: 1px solid #dde3eb;
  border-radius: 5px;
  padding: 0.08em 0.28em;
}
.article pre {
  max-width: 900px;
  margin: 22px 0;
  padding: 18px 20px;
  overflow-x: auto;
  color: var(--code-ink);
  background: var(--code-bg);
  border-radius: 8px;
  line-height: 1.5;
  font-size: 14px;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.06);
}
.article pre code {
  background: transparent;
  border: 0;
  padding: 0;
  color: inherit;
  font-size: inherit;
}
.article blockquote {
  max-width: 860px;
  margin: 24px 0;
  padding: 16px 20px;
  border-left: 5px solid var(--accent);
  background: #eef7f6;
  border-radius: 0 8px 8px 0;
}
.article img {
  display: block;
  width: min(100%, 900px);
  height: auto;
  margin: 26px 0 10px;
  padding: 12px;
  border: 1px solid var(--line);
  border-radius: 8px;
  background: #fff;
}
.article table {
  width: min(100%, 900px);
  margin: 24px 0;
  border-collapse: collapse;
  font-size: 15px;
}
.article th,
.article td {
  padding: 11px 12px;
  border-bottom: 1px solid var(--line);
  vertical-align: top;
  text-align: left;
}
.article th {
  color: #3d4652;
  background: var(--soft);
  font-size: 13px;
  font-weight: 800;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.article hr {
  max-width: 900px;
  border: 0;
  border-top: 1px solid var(--line);
  margin: 42px 0;
}
.note {
  max-width: 900px;
  margin: 0 0 28px;
  padding: 14px 16px;
  border: 1px solid #cbd7ea;
  border-left: 5px solid var(--link);
  border-radius: 8px;
  background: #f4f8ff;
  color: #263142;
  font-size: 15px;
}
.footer {
  padding: 26px 72px 46px;
  border-top: 1px solid var(--line);
  color: var(--muted);
  background: #fafbfc;
  font-size: 14px;
}
@media (max-width: 760px) {
  .top, .article, .footer {
    padding-left: 22px;
    padding-right: 22px;
  }
  .top {
    padding-top: 36px;
  }
  .top h1 {
    font-size: 38px;
  }
  .lede {
    font-size: 17px;
  }
  .article p,
  .article ul,
  .article ol {
    font-size: 16px;
  }
  .article h2 {
    font-size: 24px;
  }
  .article pre {
    font-size: 13px;
  }
}
"""


def render() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    md = markdown.Markdown(extensions=["extra", "sane_lists"])
    body = md.convert(source)
    title = source.splitlines()[0].lstrip("# ").strip()

    html = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <meta name="description" content="A postmortem on Glyph, a Rust tool-use agent experiment built around SFT and PRIME-RL RLVR.">
  <style>{CSS}</style>
</head>
<body>
  <div class="shell">
    <header class="top">
      <p class="kicker">Glyph postmortem</p>
      <h1>{title}</h1>
      <p class="lede">A Rust tool-use agent experiment: SFT built the agent, RLVR moved the sampled distribution, and strict held-out agent validity still did not improve.</p>
    </header>
    <main class="article">
      <div class="note">This HTML page is rendered directly from <a href="finalized_blogv3.md">finalized_blogv3.md</a>. Code: <a href="https://github.com/JayZenith/glyph">github.com/JayZenith/glyph</a>.</div>
{body}
    </main>
    <footer class="footer">
      Code and artifacts: <a href="https://github.com/JayZenith/glyph">github.com/JayZenith/glyph</a>.
    </footer>
  </div>
</body>
</html>
"""
    TARGET.write_text(html, encoding="utf-8")


if __name__ == "__main__":
    render()
