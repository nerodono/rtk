use std::{
    io::Write,
    net::{SocketAddr, SocketAddrV4, TcpStream},
    time::Duration,
};

use base64::{engine::general_purpose, Engine};
use reqwest::{blocking::ClientBuilder, IntoUrl};
use serde::{de::DeserializeOwned, Serialize};
use url::Host;

pub fn web_get_xml<T: DeserializeOwned>(url: impl AsRef<str>) -> T {
    web_get_text(url, |t| {
        serde_xml_rs::from_str(&t).expect("Failed to parse xml")
    })
}

pub fn b64encode(s: &str) -> String {
    general_purpose::STANDARD.encode(s)
}

pub fn send_http_post_no_response<T: Serialize>(to: impl IntoUrl, data: T) {
    let url = to.into_url().unwrap();
    let port = url.port().unwrap_or(80);
    let host = match url.host().unwrap() {
        Host::Domain(..) => unimplemented!(),
        Host::Ipv4(i) => SocketAddr::V4(SocketAddrV4::new(i, port)),
        Host::Ipv6(..) => unimplemented!(),
    };
    let mut stream = TcpStream::connect_timeout(&host, Duration::from_secs(5)).unwrap();
    let qs = serde_qs::to_string(&data).unwrap();

    stream
        .write_all(
            format!(
                concat!(
                    "POST {path} HTTP/1.1\r\n",
                    "Connection: close\r\n",
                    "Accept-Language: sus\r\n",
                    "Content-Type: application/x-www-form-urlencoded\r\n",
                    "Content-Length: {length}\r\n",
                    "\r\n",
                    "{qs}"
                ),
                path = url.path(),
                length = qs.len(),
                qs = qs
            )
            .as_bytes(),
        )
        .unwrap();
}

pub fn unspecified_empty(s: &str) -> &str {
    if s.is_empty() {
        "<Unspecified>"
    } else {
        s
    }
}

pub fn censor_if(value: bool, s: &str) -> &str {
    if value {
        "<Censored>"
    } else {
        s
    }
}

pub fn web_get_text<T: DeserializeOwned>(
    url: impl AsRef<str>,
    parser: impl FnOnce(String) -> T,
) -> T {
    parser(
        ClientBuilder::new()
            .build()
            .unwrap()
            .get(url.as_ref())
            .header("Accept-Language", "sus")
            .send()
            .expect("Failed to send request")
            .text()
            .unwrap(),
    )
}

pub fn naive_join_slash(left: &str, right: &str) -> String {
    let left = left.strip_suffix('/').unwrap_or(left);
    let right = right.strip_prefix('/').unwrap_or(right);
    format!("{left}/{right}")
}
