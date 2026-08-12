# alice-wreq/wreq-js

Node/TypeScript HTTP client (NAPI) for browser TLS impersonation.

| Layer | Source |
|-------|--------|
| **Engine** | [`alice-wreq/wreq-rust`](https://github.com/alice-wreq/wreq-rust) / local `Documents/GitHub/wreq-rust` (git dep, `branch = main`) |
| **Profiles** | `vendor/wreq-util` (includes `safari_ios_26.6` peet-aligned) |

## Upstream v3.0.0 parity (sqdshguy/wreq-js)

Synced JS/NAPI features from upstream **v3.0.0** while keeping alice engine/vendor:

- Browser family aliases (`chrome` → newest, e.g. `chrome_150`) + `resolveProfile()`
- `getEmulationHeaders()`
- `resolve` DNS map on `createTransport`
- Fetch-aligned `BodyInit` (streams / iterables)
- `engines.node >= 20`, manylinux_2_28 for linux-gnu natives
- Explicit resource management / `getSetCookie` (already present)

Not taken from upstream: crates.io `wreq`/`wreq-util` (alice git + vendor stay).

## Install (app)

**Published npm name: `@alice-doctor/wreq-js`** (not bare `wreq-js`).

```bash
npm install @alice-doctor/wreq-js
# or
bun add @alice-doctor/wreq-js
```

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

Git dependency (optional):

```json
{
  "dependencies": {
    "@alice-doctor/wreq-js": "github:alice-wreq/wreq-js#v2.3.3"
  }
}
```

```bash
bun install
# postinstall: downloads platform native from GitHub Releases if missing
# e.g. wreq-js.linux-x64-gnu.node / linux-arm64-gnu.node
```

## Develop

```bash
npm install
npm run build        # build:rust + build:ts
# requires: Rust, napi-cli
```

`rust/Cargo.toml`:

```toml
wreq = { git = "https://github.com/alice-wreq/wreq-rust", branch = "main", ... }
wreq-util = { path = "../vendor/wreq-util", ... }
```

## Flow: rust change → js binary

1. Push `alice-wreq/wreq-rust` `main`
2. Workflow `notify-wreq-js` → `repository_dispatch` to this repo  
   (needs `WREQ_JS_DISPATCH_TOKEN` PAT with `repo` on wreq-js if default token insufficient)
3. `build-and-release` rebuilds natives against latest `wreq-rust` main  
   (matrix: `darwin-arm64`, `linux-x64-gnu`, `linux-arm64-gnu`, **`win32-x64-msvc`**)
4. Artifacts uploaded to GitHub Release

## License

- wreq-js: MIT  
- wreq-rust / wreq-util: Apache-2.0 (upstream)
EOF
## npm publish (`@alice-doctor/wreq-js`)

### Automatic (preferred)

1. Bump `package.json` / `rust/Cargo.toml` version  
2. Commit, push `main`, then tag and push:

```bash
git tag -a v3.0.1 -m "v3.0.1"
git push origin v3.0.1
```

3. Workflow **`build-and-release`**:
   - builds platform natives  
   - uploads GitHub Release assets  
   - **`npm publish`** on tags matching `v*` (OIDC trusted publishing)

`main` push without a version tag only creates a prerelease `build-<sha>` (no npm).

### Manual fallback

```bash
npm login
gh release download vX.Y.Z -R alice-wreq/wreq-js -D rust --pattern 'wreq-js.*.node'
npm run build:ts
npm publish --access public
```

npm Trusted Publishing must allow this repo’s GitHub Actions for `@alice-doctor/wreq-js`  
(https://www.npmjs.com/package/@alice-doctor/wreq-js → Settings → Publishing access).

App:

```json
"@alice-doctor/wreq-js": "^3.0.0"
```

```ts
import { fetch, getProfiles } from "@alice-doctor/wreq-js";
```
