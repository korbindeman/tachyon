# Tachyon

**Tachyon** is my personal self-hosted file transfer server, built in Rust using [Actix Web](https://actix.rs/). It allows me to upload and share large files through short, clean download links.

I’ve done my best to make it **convenient, reliable, and secure**.

---

### ✨ Features

- Short, shareable download URLs
- Download count tracking
- Customizable link expiration time (🚧 WIP)
- Password protected links (🚧 WIP)

---

### 🔐 Security Considerations

- Uploads require a valid API key
- File paths are validated and canonicalized (no traversal vulnerabilities)
- Rate limiting (🚧 WIP)
- Download links are public by design — password protection is planned for sensitive files

---

### 🛰️ Usage

🚧 WIP

---

### ⚙️ Setup

There are some environment variables that need to be set up, however most of them have default values. You can put these in a `.env` file in the root directory.

| Variable           | Default             | Description |
|--------------------|---------------------|-------------|
| `HOST`             | `127.0.0.1`          | Host address. |
| `PORT`             | `8080`               | Port to bind the server on. |
| `BASE_URL`         | `http://{HOST}:{PORT}`        | The full base URL where the app is hosted (e.g. `http://localhost:8080`) used for generating download links. |
| `DATABASE_URL`     | `sqlite://sqlite.db` | URL to your SQLite database. |
| `TRANSFERS_DIR`    | `transfers/`         | Directory where uploaded files are stored. Will NOT be created if it doesn't exist. |
| `API_KEY`          | _(required)_         | Secret key required to upload files. Must be sent in `x-api-key` header. This will likely be removed in the future. |
| `ID_LENGTH`        | `5`                  | Length of generated file codes (e.g. `abcde`). |
| `PAYLOAD_LIMIT_MB` | `5120`               | Max upload size in megabytes (default: 5GB). |
| `RATE_LIMIT_RPS`   | `5`                  | Max requests per second per IP. |

Then you can run the server using the following command:

bash```
cargo run
```

For production, you should run the server using the following command:

bash```
cargo run --release
```

Or, build the binary and run it directly:

bash```
cargo build --release
./target/release/tachyon
```

---

### 🛠️ How I Host It

I run the server on a Raspberry Pi 5 4GB + an NVME SSD at home. I live in an apartment with shared internet access (which I can't control), so to expose the server to the internet, I am using Cloudflare Tunnel.

It's integrated into my personal website at [korbin.co](https://korbin.co), where I serve files with a frontend at routes like:
korbin.co/files/abcde

---

⚠️ **Disclaimer:** This is a personal project. It works great for my use case, but it’s not intended for public deployment without further security hardening. Use at your own risk.
