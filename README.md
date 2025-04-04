# Discord OAuth2 Cloudflare Worker

This Cloudflare Worker handles Discord OAuth2 login, exchanges tokens, fetches user/guild info, and issues signed JWTs — with RSA keys securely stored in **Workers KV**.

---

## 🛠 Prerequisites

- Rust
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/)
- A registered [Discord Developer Application](https://discord.com/developers/applications)

---

## 📁 Project Structure

- `src/`: Rust source code (modularized)
- `wrangler.toml`: Wrangler project config
- `.env`: Local secrets for dev testing

---

## 🔐 Environment Variables

Set these in `wrangler.toml` under `[vars]` or via the Cloudflare dashboard:

| Variable                | Description                                              |
| ----------------------- | -------------------------------------------------------- |
| `DISCORD_CLIENT_ID`     | Discord application's Client ID                          |
| `DISCORD_CLIENT_SECRET` | Discord application's Client Secret                      |
| `DISCORD_REDIRECT_URL`  | OAuth2 redirect URI (must match in Discord app settings) |

Example `.env`:

```env
DISCORD_CLIENT_ID=123456789012345678
DISCORD_CLIENT_SECRET=your_super_secret
DISCORD_REDIRECT_URL=https://yourdomain.dev/callback
```

---

## 🔐 KV Namespace

This Worker uses [Cloudflare KV](https://developers.cloudflare.com/workers/runtime-apis/kv/) to persist the RSA private key for JWT signing.

### Create KV Namespace:

```sh
wrangler kv namespace create "KEYS_STORE"
```

Copy the ID and add to `wrangler.toml`:

```toml
[[kv_namespaces]]
binding = "KEYS_STORE"
id = "paste-your-namespace-id-here"
```

---

## 🚀 Deployment Instructions

### 3. Configure `wrangler.toml`

```toml
name = "discord-oidc"
compatibility_date = "2025-04-04"
main = "build/worker/shim.mjs"

[build]
command = "cargo install -q worker-build && worker-build --release"

[vars]
DISCORD_CLIENT_ID = "your-client-id"
DISCORD_CLIENT_SECRET = "your-secret"
DISCORD_REDIRECT_URL = "https://yourdomain.dev/callback"

[[kv_namespaces]]
binding = "KEYS_STORE"
id = "your-namespace-id"
```

### 4. Publish

```sh
wrangler deploy
```

---

## 🧪 API Endpoints

| Method | Path                    | Description                            |
| ------ | ----------------------- | -------------------------------------- |
| GET    | `/authorize/:scopemode` | Begins OAuth2 flow (`email`, `guilds`) |
| POST   | `/token`                | Exchanges code for Discord data + JWT  |
| GET    | `/jwks.json`            | Returns public key in JWK-like format  |

---

## 🔐 JWT Security Notes

- RSA private key is persisted to Workers KV.
- JWT is signed using `RS256` with a `kid`.
- Public key is exposed at `/jwks.json`.
- Only Discord user/guild info is included in claims — no secrets.

---

## 🧰 Optional Dev Enhancements

- Add `.env` + `dotenvy` for local CLI testing
- Use `base64url` JWK format instead of PEM (see `/jwks.json`)
- Add rate-limiting middleware for token abuse prevention
