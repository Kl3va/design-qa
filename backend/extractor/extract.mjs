// Exploratory Design QA extractor — NOT the final Rust system.
// Usage:
//   node extract.mjs <url> --out=./output [--viewport-only] [--no-full-page]
//
// JSON payload is written to stdout. Screenshots are written under --out.
// All logging goes to stderr so stdout stays pure JSON.

import { chromium } from "playwright";
import { mkdir } from "node:fs/promises";
import path from "node:path";

function parseArgs(argv) {
  const args = {
    url: null,
    outDir: "./output",
    // viewportOnly: false,
    // fullPage: true,
  };
  for (const a of argv.slice(2)) {
    if (a.startsWith("--out=")) args.outDir = a.slice(6);
    // else if (a === "--viewport-only") args.viewportOnly = true;
    // else if (a === "--no-full-page") args.fullPage = false;
    else if (!a.startsWith("--")) args.url = a;
  }
  return args;
}

const { url, outDir } = parseArgs(process.argv);

if (!url) {
  console.error(
    "Error: no URL provided. Usage: node extract.mjs <url> --out=./output",
  );
  process.exit(1);
}

const outPath = path.resolve(outDir);
await mkdir(outPath, { recursive: true });

let browser;
try {
  browser = await chromium.launch();
  const page = await browser.newPage({
    viewport: { width: 1440, height: 900 },
  });

  console.error(`Loading ${url} ...`);
  await page.goto(url, { waitUntil: "networkidle", timeout: 30000 });
  await page.waitForTimeout(1000); // let fonts/layout settle

  const extracted = await page.evaluate(
    (opts) => {
      const SKIP_TAGS = new Set([
        "HTML",
        "HEAD",
        "META",
        "TITLE",
        "LINK",
        "SCRIPT",
        "STYLE",
        "NOSCRIPT",
        "TEMPLATE",
      ]);
      const vw = window.innerWidth;
      const vh = window.innerHeight;
      const round2 = (n) => Math.round(n * 100) / 100;

      const truncate = (s, max = 300) => {
        const t = (s ?? "").replace(/\s+/g, " ").trim();
        return t.length > max ? t.slice(0, max) + "…" : t;
      };

      const cssSelector = (el) => {
        let s = el.tagName.toLowerCase();
        if (el.id) s += `#${el.id}`;
        else if (typeof el.className === "string" && el.className.trim()) {
          s += "." + el.className.trim().split(/\s+/).slice(0, 3).join(".");
        }
        return s;
      };

      const isInViewport = (r) =>
        r.bottom > 0 && r.right > 0 && r.top < vh && r.left < vw;

      let nextId = 0;

      function extractNode(el) {
        if (SKIP_TAGS.has(el.tagName)) return null;

        const cs = getComputedStyle(el);
        if (cs.display === "none" || cs.visibility === "hidden") return null;

        const r = el.getBoundingClientRect();
        const opacity = parseFloat(cs.opacity);
        const inViewport = isInViewport(r);

        const selfValid = r.width > 0 && r.height > 0 && opacity !== 0; // && !(opts.viewportOnly && !inViewport) if i decide to capture only viewport elements

        const childResults = [];
        for (const child of el.children) {
          const result = extractNode(child);
          if (result === null) continue;
          if (Array.isArray(result)) childResults.push(...result);
          else childResults.push(result);
        }

        if (!selfValid) {
          return childResults.length ? childResults : null;
        }

        const id = nextId++;

        let ownText = "";
        for (const n of el.childNodes) {
          if (n.nodeType === 3) ownText += n.textContent;
        }

        return {
          id,
          tag: el.tagName.toLowerCase(),
          idAttr: el.id || null,
          className: typeof el.className === "string" ? el.className : null,
          selector: cssSelector(el),
          role: el.getAttribute("role"),
          text: truncate(el.innerText, 300),
          ownText: truncate(ownText, 300),
          hasText: Boolean(el.innerText && el.innerText.trim()),
          rect: {
            x: round2(r.x),
            y: round2(r.y),
            width: round2(r.width),
            height: round2(r.height),
            top: round2(r.top),
            left: round2(r.left),
            bottom: round2(r.bottom),
            right: round2(r.right),
          },
          inViewport,
          font: {
            family: cs.fontFamily,
            size: cs.fontSize,
            weight: cs.fontWeight,
            style: cs.fontStyle,
            lineHeight: cs.lineHeight,
            letterSpacing: cs.letterSpacing,
            textTransform: cs.textTransform,
            textAlign: cs.textAlign,
            textDecorationLine: cs.textDecorationLine,
          },
          color: cs.color,
          backgroundColor: cs.backgroundColor,
          opacity: cs.opacity,
          border: {
            top: `${cs.borderTopWidth} ${cs.borderTopStyle} ${cs.borderTopColor}`,
            right: `${cs.borderRightWidth} ${cs.borderRightStyle} ${cs.borderRightColor}`,
            bottom: `${cs.borderBottomWidth} ${cs.borderBottomStyle} ${cs.borderBottomColor}`,
            left: `${cs.borderLeftWidth} ${cs.borderLeftStyle} ${cs.borderLeftColor}`,
          },
          borderRadius: {
            topLeft: cs.borderTopLeftRadius,
            topRight: cs.borderTopRightRadius,
            bottomRight: cs.borderBottomRightRadius,
            bottomLeft: cs.borderBottomLeftRadius,
          },
          layout: {
            display: cs.display,
            position: cs.position,
            zIndex: cs.zIndex,
          },
          children: childResults,
        };
      }

      const result = extractNode(document.body);
      const root = Array.isArray(result)
        ? { id: -1, tag: "root", children: result }
        : (result ?? { id: -1, tag: "root", children: [] });

      function countNodes(node) {
        return (
          1 + (node.children ?? []).reduce((sum, c) => sum + countNodes(c), 0)
        );
      }
      const count =
        root.tag === "root"
          ? (root.children ?? []).reduce((sum, c) => sum + countNodes(c), 0)
          : countNodes(root);

      return { viewport: { w: vw, h: vh }, count, root };
    },
    // { viewportOnly },
  );

  const payload = {
    url,
    timestamp: new Date().toISOString(),
    viewport: extracted.viewport,
    totalRelevantVisible: extracted.count,
    root: extracted.root,
  };

  await page.screenshot({
    path: path.join(outPath, "screenshot.png"),
    fullPage: true,
  });
  // if (fullPage) {
  //   await page.screenshot({
  //     path: path.join(outPath, "screenshot-full.png"),
  //     fullPage: true,
  //   });
  // }

  // Only real output on stdout.
  process.stdout.write(JSON.stringify(payload));
} finally {
  if (browser) await browser.close();
}
