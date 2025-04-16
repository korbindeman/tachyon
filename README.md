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

### 🛠️ How I Use It

I run the server on a Raspberry Pi 5 at home. It's integrated into my personal website at [korbin.co](https://korbin.co), where I serve files with a frontend at routes like:
korbin.co/files/abcde

---

### ⚙️ Setup

🚧 WORK IN PROGRESS

---

### 🚧 Planned Features
- Customizable link expiration time
- Password protection for sensitive files
- Support for multiple file uploads (auto zip)
- Zip file contents info (for frontends)
- More file storage options (e.g., cloud storage services)
- Code quailty improvements:
  - Tests

---

⚠️ **Disclaimer:** This is a personal project. It works great for my use case, but it’s not intended for public deployment without further security hardening. Use at your own risk.
