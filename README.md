# @alice-doctor/wreq-js

[![npm](https://img.shields.io/npm/v/@alice-doctor/wreq-js.svg)](https://www.npmjs.com/package/@alice-doctor/wreq-js)
[![CI](https://github.com/alice-wreq/wreq-js/actions/workflows/build-release.yml/badge.svg)](https://github.com/alice-wreq/wreq-js/actions)
[![GitHub](https://img.shields.io/badge/github-alice--wreq%2Fwreq--js-blue)](https://github.com/alice-wreq/wreq-js)

**npm package name: `@alice-doctor/wreq-js`** (not the upstream `wreq-js`).

alice-wreq fork of Node.js/TypeScript HTTP client for TLS fingerprinting (Cloudflare / DataDome 등), powered by native Rust bindings from [alice-wreq/wreq-rust](https://github.com/alice-wreq/wreq-rust) (upstream [wreq](https://github.com/0x676e67/wreq)).

If your requests work in a browser but get blocked from Node.js because your network fingerprint looks wrong, this is for you.
You keep a fetch style API and get browser profile level network behavior without running a full browser.

1. Built in browser TLS and HTTP fingerprint profiles across Chrome, Firefox, Safari, Edge, Opera, and OkHttp families
2. Native Rust engine for high throughput traffic with no browser process overhead
3. Fetch style API with sessions, cookies, proxies, and transport controls
4. WebSocket helper and constructor APIs with session cookie and transport reuse
5. TypeScript first developer experience with generated definitions
6. Native targets for macOS, Linux, and Windows

Common search terms: cloudflare bypass, datadome bypass, tls fingerprinting, ja3, ja4, browser impersonation, nodejs fetch, typescript http client.

## Alternatives comparison

| Library | Approach | API | Notes |
|---------|----------|-----|-------|
| **@alice-doctor/wreq-js** | Rust native ([alice-wreq/wreq-rust](https://github.com/alice-wreq/wreq-rust)) | Fetch style, TypeScript first | alice profiles (e.g. `chrome_150`, `safari_ios_26.6`) |
| [CycleTLS](https://github.com/Danny-Dasilva/CycleTLS) | Go subprocess bridge | Promise based | Subprocess model |
| [got-scraping](https://github.com/apify/got-scraping) | JavaScript HTTP client customization | `got` based | Header and request customization |
| [node-tls-client](https://github.com/Sahil1337/node-tls-client) | Native shared library bindings | Custom | Behavior depends on upstream native layer |
| [curl-impersonate](https://github.com/lwthiker/curl-impersonate) | curl based tooling | CLI and bindings | Binary/tooling workflow |

## Documentation

All guides, concepts, and API reference live at:

- https://wreq.sqdsh.win

(If you're looking for examples, sessions/cookies, proxy usage, streaming, WebSockets, or the full API surface - it's all there.)

Quick links:
1. Quickstart: https://wreq.sqdsh.win/quickstart
2. API overview: https://wreq.sqdsh.win/api-reference/overview
3. Sessions: https://wreq.sqdsh.win/concepts/sessions
4. WebSockets: https://wreq.sqdsh.win/guides/websockets
5. Compatibility matrix: https://wreq.sqdsh.win/concepts/compatibility-matrix

## Installation

```bash
npm install @alice-doctor/wreq-js
# or
yarn add @alice-doctor/wreq-js
pnpm add @alice-doctor/wreq-js
bun add @alice-doctor/wreq-js
```

> **Do not** `npm install @alice-doctor/wreq-js` for this fork — that installs the **upstream** package without alice profiles / natives.
> Always use the scoped name: **`@alice-doctor/wreq-js`**.

```json
{
  "dependencies": {
    "@alice-doctor/wreq-js": "^2.3.3"
  }
}
```

```ts
import { fetch, getProfiles } from "@alice-doctor/wreq-js";
```

Prebuilt natives (GitHub Releases / postinstall):
1. macOS — `darwin-arm64` (and `darwin-x64` when published)
2. Linux — `linux-x64-gnu`, `linux-arm64-gnu` (musl when published)
3. Windows — `win32-x64-msvc` (64-bit only)

If a matching prebuilt is missing, postinstall tries [GitHub Releases](https://github.com/alice-wreq/wreq-js/releases); otherwise build from source (Rust + cmake).

See also [ALICE_WREQ.md](./ALICE_WREQ.md).

## Quick start

```ts
import { fetch } from "@alice-doctor/wreq-js";

const res = await fetch("https://example.com/api", {
  browser: "chrome_150",
  os: "macos",
});

console.log(await res.json());
```

By default, standalone `fetch()` calls use isolated ephemeral cookie storage.
Use `createSession()` when you want cookie persistence across requests.

## Use sessions (recommended)

For **most real-world workloads**, start with a session and reuse it across requests.
This keeps one cookie and request context for multi step flows.

```ts
import { createSession } from '@alice-doctor/wreq-js';

const session = await createSession({ browser: 'chrome_142', os: 'windows' });

try {
  const a = await session.fetch('https://example.com/a');
  const b = await session.fetch('https://example.com/b');
  console.log(a.status, b.status);
} finally {
  await session.close();
}
```

More session patterns: https://wreq.sqdsh.win

## WebSockets

Use the helper for a connected socket from one `await`.

```ts
import { websocket } from '@alice-doctor/wreq-js';

const ws = await websocket('wss://example.com/ws', {
  browser: 'chrome_142',
  headers: {
    Authorization: 'Bearer token',
  },
});

ws.onmessage = (event) => {
  console.log(event.data);
};

ws.send('hello');
ws.close(1000, 'done');
```

Use the constructor when you want browser like `CONNECTING` behavior.

```ts
import { WebSocket } from '@alice-doctor/wreq-js';

const ws = new WebSocket('wss://example.com/ws', {
  browser: 'chrome_142',
  os: 'windows',
});

ws.onopen = () => {
  void ws.send('connected');
};
```

Use `session.websocket(...)` to reuse cookies and transport settings from session HTTP calls.

```ts
import { createSession } from '@alice-doctor/wreq-js';

const session = await createSession({ browser: 'chrome_142' });

try {
  await session.fetch('https://example.com/login', {
    method: 'POST',
    body: new URLSearchParams({ user: 'name', pass: 'secret' }),
  });

  const ws = await session.websocket('wss://example.com/ws');
  ws.onmessage = (event) => {
    console.log(event.data);
  };
} finally {
  await session.close();
}
```

## When to use

Use `wreq-js` when your Node.js HTTP or WebSocket traffic gets blocked because of TLS fingerprinting or browser profile mismatches.
It is a good fit when you want Cloudflare bypass and DataDome bypass style network behavior with a familiar fetch style API.
It handles transport and fingerprint level behavior, not CAPTCHA solving and not in page JavaScript execution.

If you need DOM/JS execution, CAPTCHA solving, or full browser automation, use Playwright/Puppeteer instead.

## FAQ

1. Why use sessions?
Use sessions for multi-step flows where cookie and request context should be shared.

2. Why does install compile from source on some machines?
If a matching prebuilt native artifact is unavailable, npm may build from source.

3. Can I use per-request proxy overrides inside a session?
Yes, by passing a `transport` on that specific `session.fetch(...)` call. The `proxy` field itself remains session-scoped.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Origins

This is a maintained fork of [will-work-for-meal/node-wreq](https://github.com/will-work-for-meal/node-wreq) (originally named `node-wreq`), with ongoing updates, compatibility fixes, and performance work.

## Acknowledgments

- [wreq](https://github.com/0x676e67/wreq) - Rust HTTP client with browser impersonation
- [wreq-util](https://github.com/0x676e67/wreq-util) - related browser profile tooling in the upstream ecosystem
- [NAPI-RS](https://napi.rs/) - Rust ↔ Node.js bindings
