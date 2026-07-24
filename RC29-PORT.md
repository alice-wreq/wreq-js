# wreq-js fork: wreq 6.0.0-rc.29

## Changes
- `rust/Cargo.toml`: `wreq` **6.0.0-rc.28 → 6.0.0-rc.29**, `wreq-util` **3.0.0-rc.11 → 3.0.0-rc.14**
- features: `tokio-rt` via default-features; util `emulation` + `tokio-rt`
- dropped `boring2` dev-dep (conflicts with `btls`)
- API port for util rename: `Emulation`/`EmulationOS`/`EmulationOption` → `Profile`/`Platform`/`Emulation`
- wreq Emulation public fields; cookie jar `Version` arg; cert store path renames
- experimental HTTP/2 settings unsupported (skipped)

## Build
```bash
source ~/.cargo/env
cd rust && cargo build --release
cp target/release/libwreq_js.dylib wreq-js.darwin-arm64.node
codesign --force -s - wreq-js.darwin-arm64.node
```

## naver-wtm
`package.json`: `"wreq-js": "file:../wreq-js"`
