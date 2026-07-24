# alice-wreq/wreq-js

Node/TypeScript HTTP client (NAPI) for browser TLS impersonation.

| Layer | Source |
|-------|--------|
| **Engine** | [`alice-wreq/wreq-rust`](https://github.com/alice-wreq/wreq-rust) (git dep, `branch = main`) |
| **Profiles** | `vendor/wreq-util` (includes `safari_ios_26.6` peet-aligned) |

## Install (app)

```bash
# package.json
"wreq-js": "github:alice-wreq/wreq-js#main"

# or pin a tag
"wreq-js": "github:alice-wreq/wreq-js#v2.3.1-alice.1"
```

```bash
bun install
# postinstall: uses committed rust/wreq-js.<platform>.node
# or downloads from GitHub Releases if missing
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
4. Artifacts uploaded to GitHub Release

## License

- wreq-js: MIT  
- wreq-rust / wreq-util: Apache-2.0 (upstream)
EOF
## npm publish (`@alice-doctor/wreq-js`)

```bash
# 1) https://www.npmjs.com/org/create  → organization: alice-wreq
# 2) npm login
npm login

# 3) from this repo (after build)
npm run build
npm publish --access public
```

App:

```json
"@alice-doctor/wreq-js": "^2.3.1-alice.1"
```

```ts
import { fetch, getProfiles } from "@alice-doctor/wreq-js";
```
