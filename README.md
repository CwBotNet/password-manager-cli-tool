# 📖 Rust Password Manager CLI

```markdown
# 🔐 Password Manager CLI

A modern, secure, and extensible command-line password manager written in Rust. Designed to showcase best practices in cryptography, security architecture, and developer UX for industry-level applications.

---

## ✨ Features

- **Military-grade encryption:** AES-256-GCM with Argon2id key derivation
- **Zero-trust storage:** All credentials are encrypted at-rest, even metadata
- **Flexible CLI:** Add, list, search, get, delete credentials; change master password; vault statistics
- **Strong password generator:** Configurable, CSPRNG-backed, professional-grade
- **Memory safety:** Sensitive data automatically wiped from RAM
- **Auditability:** Timestamps for creation, modification, and access
- **Cross-platform:** Works on macOS, Linux, and Windows (WSL)
- **User-first experience:** Helpful prompts, confirmation for deletes, and rich error messages

---

## 🚀 Getting Started

### 1. **Install Dependencies**

You need Rust (edition >=2021):
```

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

Then clone and build:

```

git clone https://github.com/CwBotNet/password-manager-cli-tool
cd password_manager_cli
cargo build --release

```

### 2. **Usage Examples**

#### Initialize a New Vault

```

./target/release/pwdmgr init

```

#### Add a Credential

```

./target/release/pwdmgr add "Gmail Account"

```

#### List All Credentials

```

./target/release/pwdmgr list

```

#### Search by Term

```

./target/release/pwdmgr search gmail

```

#### Get a Credential (copy to clipboard)

```

./target/release/pwdmgr get gmail --copy

```

#### Delete a Credential

```

./target/release/pwdmgr delete

```

#### Change the Master Password

```

./target/release/pwdmgr change-password

```

#### See Vault Stats

```

./target/release/pwdmgr status

```

---

## 🛡️ Security Overview

- **Master password never stored:** Only Argon2id hash and per-vault salt are saved
- **Passwords encrypted with AES-256-GCM:** All credential data at rest is unreadable without your master password
- **All memory containing secrets is zeroized** on drop using Rust zeroize crate
- **Atomic file operations:** Vault files are updated in a secure, corruption-resistant way
- **Directory/file permissions locked down** (0o700 or 0o600 on Unix)

---

## 🧑‍💻 Developer Guide

**Project Structure:**

src/
├── main.rs                # CLI entrypoint and command routing
├── models/credential.rs   # Secure data structures
├── utils/crypto.rs        # Encryption, hashing, and zeroization
├── utils/generator.rs     # Secure random password generation
├── storage/file.rs        # Atomic, encrypted disk storage
└── commands/              # All CLI commands
```

**Adding Features?**

- Model changes in `models/credential.rs`
- Secure new flows with `utils/crypto.rs`
- Extend CLI in `src/commands/`

### Documentation and Tests

- `cargo test` covers core security utilities
- All sensitive functions annotated with `///` docs for quick IDE lookup

---

## 💡 Threat Model

- Designed to protect against disk theft, OS snooping, memory scraping, casual user account compromise
- Does NOT protect if attacker has your unlocked machine and master password
- Zero dependencies on Trust, Cloud, or Internet services

---

## 📚 Further Reading

- [Argon2 Password Hashing](https://github.com/P-H-C/phc-winner-argon2)
- [AES-GCM Authenticated Encryption (RFC 5116)](https://www.rfc-editor.org/rfc/rfc5116)

---

## 🎓 Why This Project Matters

- Demonstrates **real-world Rust security engineering**
- All code structured for maintainability and future additions
- Employs patterns used in $100k+ industry roles (secure storage, CLI UX, cryptography)
- Ready for extension into wallet apps, Solana programs, or commercial CLI products

## 👷 Author

Created by [Raj sahani](https://github.com/CwBotNet) — built as a professional portfolio project targeted at security, blockchain, and systems roles.

---

**Pull requests and suggestions are welcome!**

````

# 📝 In-Code Documentation

1. **Each public function (especially in `crypto.rs`, `storage.rs`, and model code) should have a `///` doc comment.**
   - Example:
     ```rust
     /// Securely generate a password with optional symbols.
     /// Returns an error if length is too small.
     pub fn generate_password(...) -> ...
     ```

2. **Module (file-level) doc comments:**
   - At the top of each main file:
     ```rust
     //! This module defines the Credential struct and vault management logic.
     ```

3. **Comment on security-sensitive design decisions.**
   - In `crypto.rs` near your Argon2id or AES derive logic, briefly explain why those settings are chosen.

# 🎯 Final Touches

- Run `cargo doc --open` to preview auto-generated Rust docs!
- Link to your README from your project repo for portfolio visibility.
- Add screenshots or terminal asciinema/GIFs to showcase usage (optional but very impressive!).
- Consider a LICENSE file (MIT/Apache-2.0 dual license is standard for open source).
````
