# vendor/wreq-util — alice-wreq patches

Base: **wreq-util 0.2.1** (crates.io, git `c4bb8e92` / tag `wreq-util-v0.2.1`).
CRLF line endings from the crates.io tarball were normalized to LF.

When bumping the base, replace `vendor/wreq-util` with the pristine crate and
re-apply the patches below (keep them in a separate commit).

## Safari iOS 26.6 (`safari_ios_26.6`)

peet (tls.peet.ws) measurement, 2026-07:

| field | value |
|---|---|
| ja3_hash | `ecdf4f49dd59effc439639da29186671` |
| ja4 | `t13d2013h2_a09f3c656075_7f0f34a4126d` |
| peetprint_hash | `62b834de729e78a9f0ebd1dd099314a7` |
| akamai | `2:0;3:100;4:2097152;9:1\|10420225\|0\|m,s,a,p` |

Files:

- `src/emulate/profile/safari/http2.rs` — `settings_order!(3)` (4 settings), `http2_options!(7)`
- `src/emulate/profile/safari/header.rs` — `header_initializer_for_ios_26_6` (ko-KR, fixed `accept-encoding` incl. zstd)
- `src/emulate/profile/safari.rs` — `safari_ios_26_6` (`tls_options!(3, CIPHER_LIST_3, SIGALGS_LIST_2, CURVES_2)`)
- `src/emulate.rs` — `SafariIos26_6 => "safari_ios_26.6"`

## Chrome 150 (`chrome_150`)

wreq-util 0.2.1 stops at `chrome_149` (upstream `main` has 150–154 but they are
unreleased and need wreq APIs newer than 0.16.3). Our definition reuses the v132
TLS/H2 family, matching the peet measurement of real Chrome 150:

| field | value |
|---|---|
| ja4 | `t13d1516h2_8daaf6152771_806a8c22fdea` |
| akamai | `1:65536;2:0;4:6291456;6:262144\|15663105\|0\|m,a,s,p` |

Files:

- `src/emulate/profile/chrome.rs` — `v150` (`v132::build_emulation`, `header_initializer_with_zstd_priority`)
- `src/emulate.rs` — `Chrome150 => "chrome_150"`, added to the random-profile Chrome class

**Known gap (verified 2026-11-11 on tls.peet.ws):** HTTP/2 (akamai) and JA4 a/b match,
but JA4 c differs — we produce `…_d8a2da3f94cd`, real Chrome 150 is `…_806a8c22fdea`.
Real Chrome 150 advertises ML-DSA signature algorithms first
(`mldsa44:mldsa65:mldsa87:` + v132 list, = upstream main's `NEW_SIGALGS_LIST`; the hash of
that list reproduces `806a8c22fdea` exactly). btls **0.5.6** (latest on crates.io) rejects
these names (`INVALID_SIGNATURE_ALGORITHM`); upstream only supports them via an unreleased
btls git revision that also breaks the wreq 0.16.3 API.

When a btls release with ML-DSA ships and wreq/wreq-util releases adopt it, switch to
upstream's `chrome_150` (`tls_options!(8, CURVES_3, NEW_SIGALGS_LIST)`) and drop this patch.
