# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1] - 2026-10-11

First release. The project was developed as `mcp-passport` and renamed to `mcp-airlock` before release: the binary, library, `MCP_AIRLOCK_*` environment variables, default client ID, keychain entries and log directory all use the new name.

### Added
- **Transparent stdio ↔ HTTP proxy** for MCP clients that cannot do OAuth themselves, with a dedicated stdout writer so concurrent responses never interleave.
- **FAPI 2.0 authorization**: PAR + PKCE S256 + DPoP, following the MCP 2026-07-28 authorization rules:
  - authorization servers must advertise PKCE S256 (`code_challenge_methods_supported`);
  - scope selection from the challenge or `scopes_supported`, with scope union on step-up;
  - RFC 9207 `iss` validation, and immediate failure on `error=` responses;
  - Client ID Metadata Documents as client IDs;
  - `dpop_jkt` in PAR requests, and DPoP nonces from both the authorization server and the MCP server;
  - `offline_access` only with `--oidc-offline-access`.
- **Discovery**: RFC 9728 protected resource metadata, then RFC 8414 / OIDC metadata, with `resource` and exact `issuer` validation. A configured `--oidc-discovery-url` takes precedence over the server's `resource_metadata`. Servers that serve AS metadata as their resource metadata keep working, with a warning.
- **The airlock**: a `401` or `403 insufficient_scope` suspends outgoing requests while re-authentication runs, then resumes them. It is driven by a credential generation: concurrent rejections share one login, a silent refresh is tried first, and a fresh token rejected right away is reported as a loop instead of opening another browser login.
- **Tokens**: silent refresh with DPoP-bound refresh tokens, also proactively before expiry.
- **Credential storage** in the OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service with a kernel-keyring cache), scoped per MCP server and bound to the issuer that granted them.
- **MCP 2026-07-28, dual-era**:
  - modern and legacy clients are both forwarded with the rules of their era;
  - the `MCP-Protocol-Version` (per message), `Mcp-Method` and `Mcp-Name` headers, with the base64 sentinel encoding; `--mcp-protocol-version` is only a fallback;
  - `x-mcp-header` → `Mcp-Param-*` mirroring, invalid-tool filtering and a HeaderMismatch retry;
  - stdio `notifications/cancelled` closes the request's response stream;
  - `subscriptions/listen` passthrough;
  - no GET stream or session ids for modern servers.
- **Streamable HTTP**:
  - `Accept: application/json, text/event-stream`;
  - SSE responses to POSTs, `202 Accepted`, JSON-RPC error bodies of non-2xx replies;
  - for legacy servers: session expiry (404), `Last-Event-ID` resumption, and a GET stream (`--remote-sse-url`, defaulting to `--remote-mcp-url`) that stops cleanly on 405.
- Failed requests get a JSON-RPC error instead of leaving the client waiting; invalid JSON gets a parse error.
- Flags: `--auth-timeout-secs`, `--allow-insecure-http`, `--oidc-issuer`, `--oidc-offline-access`, `--log-dir`, and the existing OIDC/MCP settings, all also settable through `MCP_AIRLOCK_*` environment variables.
- `Proxy::call()` and `run_with_vault()` for library use.
- Release binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64) and Windows (x86_64), with SHA-256 checksums.
- A landing page on GitHub Pages, next to the versioned API docs.
- Tests: a strict dual-era MCP 2026-07-28 test server and conformance suite, an end-to-end suite against Keycloak 26.8.0 with headless Chrome, and benchmarks of the request hot paths.

### Security
- The MCP server, discovery URL and AS endpoints must use HTTPS except on loopback; plain HTTP elsewhere needs `--allow-insecure-http`.
- The `resource_metadata` SSRF check compares full origins (scheme, host and port).
- Refresh tokens are never sent to another authorization server.
- The login callback listens on loopback only (the redirect URL must be a loopback `http` URL with a port), and checks `state` and `iss`.
- A failed login never replaces the stored DPoP key.
- Logs go to a private (`0700`) per-user directory that may not be a symlink or owned by another user.
- TLS uses rustls with the operating system's trust store.

[Unreleased]: https://github.com/ffalcinelli/mcp-airlock/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/ffalcinelli/mcp-airlock/releases/tag/v0.0.1
