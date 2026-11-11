# alice-wreq/wreq-js

Fork of [sqdshguy/wreq-js](https://github.com/sqdshguy/wreq-js), published as
**`@alice-doctor/wreq-js`**.

| Layer                  | Source                                                                                 | Base                                                               |
| ---------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| JS/TS + Neon addon     | this repo                                                                              | upstream wreq-js **v3.2.0+** (`master` @ `6576746`)                |
| Engine (`wreq`)        | [`alice-wreq/wreq-rust`](https://github.com/alice-wreq/wreq-rust), tag `alice-v0.16.3` | upstream wreq **v0.16.3**                                          |
| Profiles (`wreq-util`) | `vendor/wreq-util`                                                                     | upstream wreq-util **0.2.1** + patches (`vendor/ALICE_PATCHES.md`) |

## What differs from upstream

1. **Engine** — `rust/Cargo.toml` keeps upstream's `wreq = "0.16.3"` requirement and swaps
   in the fork via `[patch.crates-io]` (applies to vendored wreq-util too → one `wreq` build).
2. **Profiles** — `safari_ios_26.6` (peet-measured) and `chrome_150` (wreq-util 0.2.1 has
   no 150 yet). Aliases resolve `chrome → chrome_150`, `safari_ios → safari_ios_26.6`.
3. **Proxy `#force-absolute`** — `proxy: "http://proxy:8080#force-absolute"` sends HTTPS
   targets in absolute-form instead of CONNECT (wreq-rust `Proxy::force_absolute`).
   `WREQ_FORCE_ABSOLUTE_PROXY=1` enables it process-wide.
4. **Packaging** — one npm package with natives in `rust/*.node` (no `@wreq-js/binding-*`
   platform packages). `postinstall` downloads `wreq-js.<triple>.node` from the GitHub
   Release `v<version>` when missing (falls back to `latest`). `dist/` is committed so
   git-dependency installs work.
5. **CI** — `build-release.yml`: darwin-arm64, linux-x64-gnu, linux-arm64-gnu,
   win32-x64-msvc → GitHub Release; `npm publish` (OIDC) on `v*` tags. Upstream's
   `test.yml` / `test-pr.yml` are kept (branch `main`).

## Install

```bash
npm install @alice-doctor/wreq-js
```

```ts
import { fetch, getProfiles } from "@alice-doctor/wreq-js";
```

## Develop

```bash
npm install
npm run build            # build:rust + build:ts (fetches wreq-rust tag from GitHub)
npm test

# against a local wreq-rust checkout (../wreq-rust):
npx napi build --platform --release --manifest-path rust/Cargo.toml -- \
  --config 'patch.crates-io.wreq.path="../wreq-rust"'
```

wreq 0.16.3 requires **rustc ≥ 1.98**.

## Syncing with upstream

```bash
git fetch upstream            # sqdshguy/wreq-js (tags disabled: tagOpt --no-tags)
git merge upstream/master
```

- **wreq-rust**: merge upstream **release tags** only (not `main`) — wreq-util releases lag
  behind wreq `main` API changes. Tag the result `alice-vX.Y.Z` and bump the
  `[patch.crates-io]` tag here.
- **wreq-util**: replace `vendor/wreq-util` with the pristine crate, re-apply
  `vendor/ALICE_PATCHES.md`, re-check fingerprints on tls.peet.ws.

## Release

1. Bump `package.json` + `rust/Cargo.toml` version, `npm run build:ts`, commit `dist/`.
2. `git tag -a vX.Y.Z -m vX.Y.Z && git push origin main vX.Y.Z`
3. `build-and-release` builds natives, creates the Release, publishes npm.

## License

wreq-js: MIT · wreq-rust / wreq-util: Apache-2.0 (upstream)
