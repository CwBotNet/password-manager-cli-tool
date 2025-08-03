# 📖 Rust Password Manager CLI

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
```
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

# 🔐 Secure Password Manager CLI
A **production-grade, industry-standard** command-line password manager built in Rust, demonstrating advanced systems programming, cryptographic security, and professional software architecture. Designed to showcase the kind of engineering expertise valued at companies like Solana Labs, Magic Eden, and other leading Web3 firms.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

[![Security](https://img.shields.io/badge/security-bank--grade-green.svg)](https://github.com/CwBotNet/password-manager-cli-tool)

[![Architecture](https://img.shields.io/badge/architecture-production--ready-blue.svg)](https://github.com/CwBotNet/password-manager-cli-tool)

---

## ✨ **Key Features**

### 🏛️ **Bank-Grade Security**
- **AES-256-GCM encryption** with authenticated encryption
- **Argon2id key derivation** (industry standard, 16-byte salts)
- **Memory safety** with automatic zeroization of sensitive data
- **Atomic file operations** preventing corruption during saves
- **Master password verification** with secure hashing

### 🛠️ **Professional CLI Experience**
- **Intuitive command structure** with helpful error messages
- **Interactive prompts** with secure password input (hidden)
- **Clipboard integration** for seamless password copying
- **Advanced search** across titles, usernames, URLs, and tags
- **Confirmation prompts** for destructive operations

### 🏗️ **Production Architecture**
- **Modular, layered design** separating concerns cleanly
- **Comprehensive error handling** with user-friendly messages
- **Cross-platform compatibility** (macOS, Linux, Windows)
- **Zero-warning compilation** demonstrating code quality
- **Extensible structure** ready for additional features

---

## 🚀 **Quick Start**

### **Prerequisites**
- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- Git for cloning the repository

### **Installation & Setup**

```
# Clone the repository
git clone https://github.com/CwBotNet/password-manager-cli-tool
cd password_manager_cli
```
# Build the application
```
cargo build
```
# Initialize your secure vault
```
./target/debug/password_manager_cli init
```
---

## 📖 **Complete Usage Guide**

### **1. Initialize Your Secure Vault**

# Create a new encrypted password vault
```
./target/debug/password_manager_cli init
```
`````
# Example output:
# 🔐 Initializing new password vault...
# Enter master password: [hidden]
# Confirm master password: [hidden]
# 🎉 Vault initialized successfully!
# 📍 Location: /Users/yourname/.password_manager/vault.vault
`````
### **2. Add Your First Credential**

# Add with interactive prompts
```
./target/debug/password_manager_cli add "Gmail Account"
```
# Add with command-line options
```
./target/debug/password_manager_cli add "GitHub" --username "johndoe" --url "https://github.com"
```
# Generate a strong password automatically
```
./target/debug/password_manager_cli add "Bank Account" --generate
```
### **3. List All Your Credentials**

# List credentials (passwords hidden)

```
./target/debug/password_manager_cli list
```
# Show passwords in output (use carefully!)
```
./target/debug/password_manager_cli list --show-passwords
```
### **4. Retrieve Specific Credentials**

# Search by title (case-insensitive)
```
./target/debug/password_manager_cli get gmail
```
# Get by exact UUID
```
./target/debug/password_manager_cli get 550e8400-e29b-41d4-a716-446655440000
```

# Copy password directly to clipboard
```
./target/debug/password_manager_cli get gmail --copy
```
### **5. Search Through Your Vault**

# Search across all fields
```
./target/debug/password_manager_cli search "google"
./target/debug/password_manager_cli search "work"
./target/debug/password_manager_cli search "github.com"
```
### **6. Delete Credentials Safely**

# Delete with confirmation prompt
```
./target/debug/password_manager_cli delete gmail
```
# Force delete without confirmation
```
./target/debug/password_manager_cli delete 550e8400-e29b-41d4-a716-446655440000 --force
```
### **7. Change Your Master Password**

# Securely re-encrypt entire vault with new password
```
./target/debug/password_manager_cli change-password
```
# Prompts for:
### - Current master password
### - New master password  
### - Confirmation of new password



### **8. View Vault Statistics**

# See vault info and statistics
```
./target/debug/password_manager_cli status
```
# Example output:
## 📊 Vault Statistics:
###    -  Total credentials: 15
###    -  Vault version: 1
###   -  Last updated: 2024-08-03 22:30:15 UTC
###    -  Created on: 2024-08-01 10:15:30 UTC


---
## 🏗️ **System Architecture**

### **Security-First Design**
```

┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   CLI Commands  │───▶│  Business Logic  │───▶│ Encrypted Storage│
│   (User Input)  │    │ (Credential Mgmt)│    │  (AES-256-GCM)   │
└─────────────────┘    └──────────────────┘    └──────────────────┘
                                 │
                       ┌──────────────────┐
                       │ Cryptographic    │
                       │ Security Layer   │
                       │ (Argon2id + AES) │
                       └──────────────────┘
```

### **Key Components**

| **Module** | **Responsibility** | **Security Features** |
|------------|-------------------|--------------------|
| `commands/` | CLI interface & user interaction | Input validation, secure prompts |
| `models/` | Data structures & business logic | Memory-safe credential handling |
| `utils/crypto.rs` | Cryptographic operations | Argon2id KDF, AES-256-GCM, zeroization |
| `storage/file.rs` | Encrypted persistence | Atomic saves, strict permissions |

---

## 🔒 **Security Specifications**

### **Encryption Standards**
- **Symmetric Encryption:** AES-256-GCM (NIST approved, industry standard)
- **Key Derivation:** Argon2id with 32-byte output, 16-byte salts
- **Authentication:** GCM mode provides built-in authentication
- **Random Generation:** Cryptographically secure random number generation

### **Memory Security**
- **Zeroization:** All sensitive data wiped from memory on drop
- **No Swapping:** Sensitive operations minimize memory exposure
- **Stack Protection:** Keys never stored in long-lived heap allocations

### **File System Security**
- **Permissions:** Vault files restricted to user-only access (0o600)
- **Directory Security:** Vault directory protected (0o700)  
- **Atomic Operations:** Prevents corruption during concurrent access

### **Threat Model Protection**
✅ **Protects Against:** Disk theft, OS compromise, memory dumps, casual access  
⚠️ **Does Not Protect Against:** Unlocked machine with master password, advanced malware

---

## 🛠️ **Development & Building**

### **Development Setup**

```
# Clone and enter directory
git clone https://github.com/CwBotNet/password-manager-cli-tool
cd password_manager_cli

# Install dependencies and build
cargo build

# Run tests
cargo test

# Check for issues
cargo clippy
```

### **Build Options**

```
# Development build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Build with clipboard support (default)
cargo build --features clipboard

# Minimal build (no clipboard)
cargo build --no-default-features
```

---

## 🧪 **Testing & Quality Assurance**

```
# Run all tests
cargo test

# Test with verbose output
cargo test -- --nocapture

# Security-focused testing
cargo audit                    # Check for vulnerabilities
cargo clippy -- -W clippy::all # Comprehensive linting
```

---

## 📁 **Project Structure**

```
password_manager_cli/
├── src/
│   ├── main.rs              # CLI entry point & routing
│   ├── models/
│   │   └── credential.rs    # Secure data structures
│   ├── utils/
│   │   ├── crypto.rs        # Cryptographic operations
│   │   └── generator.rs     # Password generation
│   ├── storage/
│   │   └── file.rs         # Encrypted file operations
│   └── commands/
│       ├── init.rs         # Vault initialization
│       ├── add.rs          # Add credentials
│       ├── list.rs         # List credentials
│       ├── get.rs          # Retrieve credentials
│       ├── delete.rs       # Remove credentials
│       ├── search.rs       # Search functionality
│       ├── change_password.rs # Master password change
│       └── status.rs       # Vault statistics
├── Cargo.toml              # Dependencies & metadata
└── README.md               # This file
```

---

## 🎯 **Professional Portfolio Value**

This password manager demonstrates **industry-grade engineering skills** valuable for:

### **$100k+ Remote Positions**
- **Systems Programming:** Low-level Rust, memory management, security
- **Cryptographic Engineering:** Industry-standard algorithms and practices  
- **CLI/UX Design:** Professional command-line interface development
- **Production Architecture:** Modular, testable, maintainable code structure

### **Web3 & Blockchain Roles**
- **Security-First Mindset:** Essential for DeFi, wallet, and protocol development
- **Rust Expertise:** Primary language for Solana, Substrate, and other chains
- **Error Handling:** Critical for transaction processing and user fund safety
- **Performance Focus:** Necessary for high-throughput blockchain applications

### **Companies & Roles This Targets**
- **Solana Labs** - Protocol Engineers ($160k+)
- **Magic Eden** - Web3 Security Developers ($140k+)
- **Phantom Wallet** - Application Security Engineers ($150k+)
- **Orca Protocol** - DeFi Security Engineers ($130k+)

---

## 🚀 **Future Enhancements**

**Planned Features:**
- 📱 **Mobile companion app** with secure sync
- 🌐 **Browser extension** for auto-fill
- 🏷️ **Advanced tagging system** for organization
- 📊 **Security audit reports** and password strength analysis
- 🔄 **Encrypted backup/restore** with cloud storage options
- 👥 **Multi-user support** for teams and families

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](https://github.com/CwBotNet/password-manager-cli-tool/blob/main/LICENSE) file for details.

---

## 🙋 **About the Developer**

Built by **[Your Name]** as a demonstration of production-ready systems programming and cryptographic security expertise. This project showcases the kind of engineering skills and attention to detail required for senior-level positions in Web3, fintech, and security-focused software development.

**Skills Demonstrated:**
- Advanced Rust programming and memory safety
- Cryptographic protocol implementation
- Production-grade error handling and user experience
- Professional software architecture and testing
- Industry-standard security practices

---

## 🔗 **Connect & Learn More**

- **GitHub:** [github.com/CwBotNet](https://github.com/CwBotNet)
- **LinkedIn:** [linkedin.com/in/raj-sahani1](https://www.linkedin.com/in/raj-sahani1)
- **Portfolio:** [rajsahani.dev](https://personal-portfolio-theta-rouge.vercel.app)

---