// Exploratory Design QA extractor — NOT the final Rust system.
// Usage:
//   node extract.mjs [url] [--out=./output] [--limit=50] [--viewport-only] [--no-full-page]
//
// Full-page screenshot is ON by default; pass --no-full-page to capture
// only the initial viewport.
//
// Example:
//   node extract.mjs https://audiophilec.netlify.app/
//
// What it does:
//   1. Loads target URL in Chromium (1440x900)
//   2. Walks the DOM RECURSIVELY, building a nested tree (not a flat list)
//   3. For each visible node: tag, text, rect, fonts, colors, borders, radius, opacity, children
//   4. Invisible/zero-size wrapper nodes are skipped, but their valid children are
//      promoted up to the nearest valid ancestor (so nothing visible gets silently dropped)
//   5. Saves JSON + screenshots to ./output/

import { chromium } from "playwright";
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

const DEFAULT_URL = "https://audiophilec.netlify.app/";

function parseArgs(argv) {
  const args = {
    url: DEFAULT_URL,
    outDir: "./output",
    limit: 30,
    viewportOnly: false,
    fullPage: true,
  };
  for (const a of argv.slice(2)) {
    if (a.startsWith("--out=")) args.outDir = a.slice(6);
    else if (a.startsWith("--limit=")) args.limit = Number(a.slice(8));
    else if (a === "--viewport-only") args.viewportOnly = true;
    else if (a === "--no-full-page") args.fullPage = false;
    else if (!a.startsWith("--")) args.url = a;
  }
  return args;
}

const { url, outDir, limit, viewportOnly, fullPage } = parseArgs(process.argv);
const outPath = path.resolve(outDir);
await mkdir(outPath, { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });

console.log(`Loading ${url} ...`);
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

    // Recursively walks `el` and its children, returning either:
    //   - a node object (el itself is valid)
    //   - an array of node objects (el itself is invalid, but has valid descendants
    //     that get "promoted" up to whichever ancestor IS valid)
    //   - null (el and everything under it is invalid/invisible)
    function extractNode(el) {
      if (SKIP_TAGS.has(el.tagName)) return null;

      const cs = getComputedStyle(el);

      // display:none / visibility:hidden hides the whole subtree —
      // nothing underneath can be visible either, so stop here entirely.
      if (cs.display === "none" || cs.visibility === "hidden") return null;

      const r = el.getBoundingClientRect();
      const opacity = parseFloat(cs.opacity);
      const inViewport = isInViewport(r);

      const selfValid =
        r.width > 0 &&
        r.height > 0 &&
        opacity !== 0 &&
        !(opts.viewportOnly && !inViewport);

      // Recurse regardless of whether `el` itself is valid — a zero-size
      // or display:contents wrapper can still have valid, visible children.
      const childResults = [];
      for (const child of el.children) {
        const result = extractNode(child);
        if (result === null) continue;
        if (Array.isArray(result))
          childResults.push(...result); // promoted grandchildren
        else childResults.push(result);
      }

      if (!selfValid) {
        // This node doesn't exist in the output tree — but its valid
        // descendants get spliced up to whichever ancestor IS valid.
        return childResults.length ? childResults : null;
      }

      const id = nextId++;

      // ownText = direct text nodes only (distinguishes wrappers from leaf text)
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
    // Normalize the top-level result to always be a single root node,
    // even in the unlikely case that <body> itself is invalid and we
    // got back an array of promoted top-level children instead.
    const root = Array.isArray(result)
      ? { id: -1, tag: "root", children: result }
      : (result ?? { id: -1, tag: "root", children: [] });

    // Flatten for a total count, just for the summary log.
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
  { viewportOnly },
);

const payload = {
  url,
  timestamp: new Date().toISOString(),
  viewport: extracted.viewport,
  totalRelevantVisible: extracted.count,
  root: extracted.root,
};

await writeFile(
  path.join(outPath, "extract.json"),
  JSON.stringify(payload, null, 2),
);
await page.screenshot({
  path: path.join(outPath, "screenshot.png"),
  fullPage: false,
});
if (fullPage) {
  await page.screenshot({
    path: path.join(outPath, "screenshot-full.png"),
    fullPage: true,
  });
}
await browser.close();

// Console preview: walk the tree depth-first, print up to `limit` nodes.
// console.log(
//   `\nDone. Relevant visible elements: ${payload.totalRelevantVisible}`,
// );
// console.log(`JSON      -> ${path.join(outPath, "extract.json")}`);
// console.log(
//   `Viewport  -> ${path.join(outPath, "screenshot.png")}` +
//     (fullPage
//       ? `\nFull page -> ${path.join(outPath, "screenshot-full.png")}`
//       : ""),
// );
// console.log(`\nPreview (first ${limit}, depth-first):\n`);

// let printed = 0;
// function printTree(node, depth) {
//   if (printed >= limit) return;
//   if (node.tag !== "root") {
//     printed++;
//     const label = node.text || node.ownText || "(no text)";
//     const pad = "  ".repeat(depth);
//     console.log(
//       `${pad}#${node.id} <${node.tag}> ${node.selector} | ${node.rect.width}x${node.rect.height} @(${node.rect.x},${node.rect.y}) ` +
//         `| font=${node.font.size}/${node.font.weight} ${node.font.family.split(",")[0]} ` +
//         `| color=${node.color} bg=${node.backgroundColor} opacity=${node.opacity} ` +
//         `| radius=${node.borderRadius.topLeft} | children=${node.children.length} ` +
//         `| "${label.slice(0, 80)}"`,
//     );
//   }
//   for (const child of node.children ?? []) {
//     if (printed >= limit) break;
//     printTree(child, node.tag === "root" ? depth : depth + 1);
//   }
// }
// printTree(payload.root, 0);

// if (payload.totalRelevantVisible > limit) {
//   console.log(
//     `... +${payload.totalRelevantVisible - limit} more (see extract.json)`,
//   );
// }
