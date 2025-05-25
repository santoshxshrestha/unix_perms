# unix_perms

A lightweight Rust library for displaying Unix-style file permissions like `-rwxr-xr--`, mimicking the output of `ls -l`.

## 🧾 Features

- Converts file mode bits (from `std::fs::Metadata`) into symbolic `rwx` string format
- Supports regular files, directories, symlinks, and other file types
- No unnecessary dependencies — pure and minimal

## 🔧 Example

```rust
use unix_perms::display_permissions;
use std::fs::metadata;

fn main() -> std::io::Result<()> {
    let meta = metadata("some_file.txt")?;
    let mode_str = display_permissions(&meta);
    println!("{}", mode_str); // Output: -rw-r--r--
    Ok(())
}
```

## 📦 Installation

Add this to your Cargo.toml:

```tolm
[dependencies]
unix_perms = "0.1.0"
```

or you can directly use cargo

```bash
cargo install unix_perms
```

## 📄 License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
