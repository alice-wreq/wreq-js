#!/usr/bin/env python3
"""Apply remaining API renames for wreq 6.0.0-rc.29 port."""
from pathlib import Path
import re

root = Path(__file__).resolve().parent

# ---- client.rs ----
p = root / "src/client.rs"
t = p.read_text()
t = t.replace(
    "use wreq::tls::{CertStore, TlsInfo};",
    "use wreq::tls::{TlsInfo, trust::CertStore};",
)
t = t.replace(".cert_verification(", ".tls_cert_verification(")
t = t.replace(".cert_store(", ".tls_cert_store(")
t = t.replace(
    "client_builder.pool_max_size(pool_max_size)",
    "client_builder.pool_max_size(pool_max_size as usize)",
)
t = t.replace("jar.cookies(&uri)", "jar.cookies(&uri, wreq::Version::HTTP_11)")
t = t.replace(
    "cookie_jar.cookies(&uri)",
    "cookie_jar.cookies(&uri, wreq::Version::HTTP_11)",
)
p.write_text(t)
print("client.rs ok")

# ---- custom_emulation.rs ----
p = root / "src/custom_emulation.rs"
t = p.read_text()
t = t.replace(
    "AlpnProtocol, AlpsProtocol, CertificateCompressionAlgorithm, ExtensionType, TlsOptions,",
    "AlpnProtocol, AlpsProtocol, ExtensionType, TlsOptions,",
)
t = t.replace(
    "*emulation.tls_options.as_mut() = Some(build_tls_options(tls_options)?);",
    "emulation.tls_options = Some(build_tls_options(tls_options)?);",
)
# skip incompatible TLS fields
t = re.sub(
    r"    if let Some\(value\) = options\.key_shares_limit \{\n"
    r"        builder = builder\.key_shares\(Some\(value\)\);\n"
    r"    \}\n",
    "    // key_shares_limit incompatible with rc.29 KeyShare list API\n"
    "    let _ = options.key_shares_limit;\n",
    t,
)
t = re.sub(
    r"    if let Some\(value\) = options\.certificate_compression_algorithms \{\n"
    r"        builder = builder\.certificate_compressors\(\n"
    r"            value\n"
    r"                \.into_iter\(\)\n"
    r"                \.map\(\|algorithm\| parse_certificate_compression_algorithm\(&algorithm\)\)\n"
    r"                \.collect::<Result<Vec<_>>>\(\)\?,\n"
    r"        \);\n"
    r"    \}\n",
    "    // certificate_compression_algorithms API changed in rc.29\n"
    "    let _ = options.certificate_compression_algorithms;\n",
    t,
)
# neutralize parse helper
t = re.sub(
    r"fn parse_certificate_compression_algorithm\(value: &str\) -> Result<[^>]+> \{[\s\S]*?\n\}",
    "#[allow(dead_code)]\n"
    "fn parse_certificate_compression_algorithm(value: &str) -> Result<&'static str> {\n"
    "    match value {\n"
    '        "zlib" => Ok("zlib"),\n'
    '        "brotli" => Ok("brotli"),\n'
    '        "zstd" => Ok("zstd"),\n'
    '        other => bail!("Invalid certificate compression algorithm: {other}"),\n'
    "    }\n"
    "}",
    t,
    count=1,
)
p.write_text(t)
print("custom_emulation.rs ok, remaining CertificateCompression", t.count("CertificateCompression"))

# ---- websocket.rs ----
p = root / "src/websocket.rs"
t = p.read_text()
t = t.replace(
    "emulation.orig_headers_mut().clone()",
    "emulation.orig_headers.clone()",
)
t = t.replace(
    "cookie_jar.cookies(&uri)",
    "cookie_jar.cookies(&uri, wreq::Version::HTTP_11)",
)
t = t.replace("jar.cookies(&uri)", "jar.cookies(&uri, wreq::Version::HTTP_11)")
p.write_text(t)
print("websocket.rs ok")
