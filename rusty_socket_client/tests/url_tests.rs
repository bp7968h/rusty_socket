use rusty_socket_client::{ScError, WebSocketUrl};

#[test]
fn test_valid_url_with_all_parts() {
    let url = "ws://example.com/path/to/resource?query=value#fragment";
    let wsu = WebSocketUrl::from_url(url).expect("Failed to parse valid URL");

    assert_eq!(wsu.scheme, "ws");
    assert_eq!(wsu.host, "example.com");
    assert_eq!(wsu.path, Some("path/to/resource".to_string()));
    assert_eq!(wsu.query, Some("query=value".to_string()));
    assert_eq!(wsu.fragment, Some("fragment".to_string()));
}

#[test]
fn test_valid_url_without_fragment() {
    let url = "wss://example.com/path/to/resource?query=value";
    let wsu = WebSocketUrl::from_url(url).expect("Failed to parse valid URL");

    assert_eq!(wsu.scheme, "wss");
    assert_eq!(wsu.host, "example.com");
    assert_eq!(wsu.path, Some("path/to/resource".to_string()));
    assert_eq!(wsu.query, Some("query=value".to_string()));
    assert_eq!(wsu.fragment, None);
}

#[test]
fn test_valid_url_without_query_and_fragment() {
    let url = "ws://example.com/path/to/resource";
    let wsu = WebSocketUrl::from_url(url).expect("Failed to parse valid URL");

    assert_eq!(wsu.scheme, "ws");
    assert_eq!(wsu.host, "example.com");
    assert_eq!(wsu.path, Some("path/to/resource".to_string()));
    assert_eq!(wsu.query, None);
    assert_eq!(wsu.fragment, None);
}

#[test]
fn test_valid_url_with_host_only() {
    let url = "ws://example.com";
    let wsu = WebSocketUrl::from_url(url).expect("Failed to parse valid URL");

    assert_eq!(wsu.scheme, "ws");
    assert_eq!(wsu.host, "example.com");
    assert_eq!(wsu.path, None);
    assert_eq!(wsu.query, None);
    assert_eq!(wsu.fragment, None);
}

#[test]
fn test_invalid_url_missing_scheme() {
    let url = "example.com/path/to/resource";
    let err = WebSocketUrl::from_url(url).expect_err("Expected error for missing scheme");

    assert_eq!(err, ScError::InvalidUrl);
}

#[test]
fn test_invalid_url_missing_host() {
    let url = "ws:///path/to/resource";
    let err = WebSocketUrl::from_url(url).expect_err("Expected error for missing host");

    assert_eq!(err, ScError::InvalidUrl);
}

#[test]
fn test_invalid_url_empty() {
    let url = "";
    let err = WebSocketUrl::from_url(url).expect_err("Expected error for empty URL");

    assert_eq!(err, ScError::InvalidUrl);
}

#[test]
fn test_invalid_url_missing_host_and_path() {
    let url = "ws:///";
    let err = WebSocketUrl::from_url(url).expect_err("Expected error for missing host and path");

    assert_eq!(err, ScError::InvalidUrl);
}

#[test]
fn test_valid_url_with_empty_query_and_fragment() {
    let url = "ws://example.com/path/to/resource?#";
    let wsu = WebSocketUrl::from_url(url).expect("Failed to parse valid URL");

    assert_eq!(wsu.scheme, "ws");
    assert_eq!(wsu.host, "example.com");
    assert_eq!(wsu.path, Some("path/to/resource".to_string()));
    assert_eq!(wsu.query, None);
    assert_eq!(wsu.fragment, None);
}
