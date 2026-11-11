#!/usr/bin/env node
/**
 * postinstall: ensure platform-native addon is present.
 * 1) already in package (rust/wreq-js.<platform>.node)
 * 2) else download from GitHub Releases (alice-wreq/wreq-js)
 * 3) else print build hint (requires Rust)
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import https from "node:https";
import { createWriteStream } from "node:fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const rustDir = path.join(root, "rust");

function platformTriple() {
  const { platform, arch } = process;
  if (platform === "darwin" && arch === "arm64") return "darwin-arm64";
  if (platform === "darwin" && arch === "x64") return "darwin-x64";
  if (platform === "linux" && arch === "x64") {
    // prefer gnu; musl users can set WREQ_JS_TRIPLE
    return process.env.WREQ_JS_TRIPLE || "linux-x64-gnu";
  }
  if (platform === "linux" && arch === "arm64") {
    return process.env.WREQ_JS_TRIPLE || "linux-arm64-gnu";
  }
  if (platform === "win32" && arch === "x64") return "win32-x64-msvc";
  return null;
}

function existsNative(triple) {
  // Prefer platform-specific name. Generic wreq-js.node is only accepted when
  // no other platform files exist (avoid darwin .node masking linux installs).
  const specific = path.join(rustDir, `wreq-js.${triple}.node`);
  if (fs.existsSync(specific)) return specific;
  const generic = path.join(rustDir, "wreq-js.node");
  if (!fs.existsSync(generic)) return undefined;
  try {
    const others = fs.readdirSync(rustDir).filter((n) => /^wreq-js\..+\.node$/.test(n));
    // foreign platform binaries present → do not treat generic as ok
    if (others.length > 0 && !others.includes(`wreq-js.${triple}.node`)) {
      return undefined;
    }
  } catch {
    /* */
  }
  return generic;
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const follow = (u, redirects = 0) => {
      if (redirects > 5) return reject(new Error("too many redirects"));
      https
        .get(u, (res) => {
          if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
            return follow(res.headers.location, redirects + 1);
          }
          if (res.statusCode !== 200) {
            reject(new Error(`HTTP ${res.statusCode} for ${u}`));
            res.resume();
            return;
          }
          fs.mkdirSync(path.dirname(dest), { recursive: true });
          const out = createWriteStream(dest);
          res.pipe(out);
          out.on("finish", () => out.close(() => resolve(dest)));
          out.on("error", reject);
        })
        .on("error", reject);
    };
    follow(url);
  });
}

async function main() {
  // skip in CI when building
  if (process.env.WREQ_JS_SKIP_POSTINSTALL === "1") {
    console.log("[wreq-js] postinstall skipped (WREQ_JS_SKIP_POSTINSTALL=1)");
    return;
  }

  const triple = platformTriple();
  if (!triple) {
    console.warn(
      `[wreq-js] unsupported platform ${process.platform}/${process.arch}; build from source: npm run build:rust`,
    );
    return;
  }

  const existing = existsNative(triple);
  if (existing) {
    console.log(`[wreq-js] native ok: ${path.relative(root, existing)}`);
    return;
  }

  const dest = path.join(rustDir, `wreq-js.${triple}.node`);
  const asset = `wreq-js.${triple}.node`;
  // Match the native addon to this package version (release tag v<version>) so the
  // JS wrapper and the binary never drift. Fall back to the latest release only when
  // the versioned release has no asset (e.g. installing an untagged git commit).
  const tags = process.env.WREQ_JS_RELEASE_TAG ? [process.env.WREQ_JS_RELEASE_TAG] : [`v${packageVersion()}`, "latest"];

  for (const tag of tags) {
    const url =
      tag === "latest"
        ? `https://github.com/alice-wreq/wreq-js/releases/latest/download/${asset}`
        : `https://github.com/alice-wreq/wreq-js/releases/download/${tag}/${asset}`;

    console.log(`[wreq-js] downloading ${asset} (${tag})…`);
    try {
      await download(url, dest);
      fs.chmodSync(dest, 0o755);
      console.log(`[wreq-js] installed ${path.relative(root, dest)}`);
      return;
    } catch (e) {
      fs.rmSync(dest, { force: true });
      console.warn(`[wreq-js] download failed (${tag}): ${e instanceof Error ? e.message : e}`);
    }
  }
  console.warn("[wreq-js] build from source (requires Rust + napi):\n  npm run build:rust");
}

function packageVersion() {
  try {
    return JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8")).version;
  } catch {
    return "0.0.0";
  }
}

main().catch((e) => {
  console.warn("[wreq-js] postinstall error:", e);
  // do not fail install
  process.exit(0);
});
