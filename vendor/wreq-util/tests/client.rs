#![cfg(not(target_arch = "wasm32"))]
mod support;

use support::server;
use wreq::Client;
use wreq_util::{Emulation, Platform, Profile};

#[tokio::test]
async fn test_client_emulation_device() {
    let server = server::http(move |req| async move {
        for (name, value) in req.headers() {
            if name == "user-agent" {
                assert_eq!(
                    value,
                    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
                );
            }
            if name == "sec-ch-ua" {
                assert_eq!(
                    value,
                    r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#
                );
            }
            if name == "sec-ch-ua-mobile" {
                assert_eq!(value, "?0");
            }
            if name == "sec-ch-ua-platform" {
                assert_eq!(value, "\"Linux\"");
            }
        }
        http::Response::default()
    });

    let url = format!("http://{}/ua", server.addr());
    let res = Client::builder()
        .emulation(
            Emulation::builder()
                .profile(Emulation::Chrome133)
                .platform(Platform::Linux)
                .http2(true)
                .build(),
        )
        .build()
        .expect("Unable to build client")
        .get(&url)
        .send()
        .await
        .expect("request");

    assert_eq!(res.status(), wreq::StatusCode::OK);
}

/// Starts a server that echoes the received `user-agent` and `sec-ch-ua`.
fn echo_ua_server() -> server::Server {
    server::http(move |req| async move {
        let header = |name| {
            req.headers()
                .get(name)
                .map(|value| value.to_str().unwrap().to_owned())
                .unwrap_or_default()
        };
        let body = format!("{}\n{}", header("user-agent"), header("sec-ch-ua"));
        http::Response::new(body.into())
    })
}

/// Returns the `user-agent` and `sec-ch-ua` a profile sends on a platform.
async fn sent_ua(
    server: &server::Server,
    profile: Profile,
    platform: Platform,
) -> (String, String) {
    let text = Client::builder()
        .emulation(
            Emulation::builder()
                .profile(profile)
                .platform(platform)
                .build(),
        )
        .build()
        .expect("client")
        .get(format!("http://{}/", server.addr()))
        .send()
        .await
        .expect("request")
        .text()
        .await
        .expect("body");
    let (ua, sec_ch_ua) = text.split_once('\n').expect("echo body");
    (ua.to_owned(), sec_ch_ua.to_owned())
}

const DESKTOP: [Platform; 3] = [Platform::MacOS, Platform::Windows, Platform::Linux];

#[tokio::test]
async fn test_edge_user_agent_has_reduced_edg_token() {
    let server = echo_ua_server();
    for (profile, major) in [
        (Emulation::Edge134, 134),
        (Emulation::Edge135, 135),
        (Emulation::Edge136, 136),
        (Emulation::Edge137, 137),
        (Emulation::Edge138, 138),
        (Emulation::Edge139, 139),
        (Emulation::Edge140, 140),
        (Emulation::Edge141, 141),
        (Emulation::Edge146, 146),
        (Emulation::Edge147, 147),
    ] {
        for platform in DESKTOP {
            let (ua, _) = sent_ua(&server, profile, platform).await;
            let tail = format!(" Safari/537.36 Edg/{major}.0.0.0");
            assert!(ua.ends_with(&tail), "{profile:?} {platform:?}: {ua}");
        }
        if major <= 141 {
            let (ua, _) = sent_ua(&server, profile, Platform::Android).await;
            let tail = format!(" Mobile Safari/537.36 EdgA/{major}.0.0.0");
            assert!(ua.ends_with(&tail), "{profile:?} Android: {ua}");
        }
    }
}

#[tokio::test]
async fn test_firefox139_rv_matches_version() {
    let server = echo_ua_server();
    for platform in DESKTOP {
        let (ua, _) = sent_ua(&server, Emulation::Firefox139, platform).await;
        assert!(
            ua.contains("rv:139.0) Gecko/20100101 Firefox/139.0"),
            "{platform:?}: {ua}"
        );
    }
}

#[tokio::test]
async fn test_safari17_2_1_version_token() {
    let server = echo_ua_server();
    let (ua, _) = sent_ua(&server, Emulation::Safari17_2_1, Platform::MacOS).await;
    assert!(ua.contains("Version/17.2.1 Safari/605.1.15"), "{ua}");
}

#[tokio::test]
async fn test_user_agent_literals_are_well_formed() {
    let server = echo_ua_server();
    for &profile in Profile::VARIANTS {
        for &platform in Platform::VARIANTS {
            let (ua, sec_ch_ua) = sent_ua(&server, profile, platform).await;
            assert!(!ua.contains("(Linux: "), "{profile:?} {platform:?}: {ua}");
            assert!(
                sec_ch_ua.is_empty()
                    || (sec_ch_ua.ends_with('"') && sec_ch_ua.matches('"').count() % 2 == 0),
                "{profile:?} {platform:?}: unbalanced sec-ch-ua {sec_ch_ua}"
            );
        }
    }
}
