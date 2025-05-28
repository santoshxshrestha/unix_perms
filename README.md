# unix_perms

A lightweight Rust library for displaying Unix-style file permissions like `-rwxr-xr--`, mimicking the output of `ls -l`.

## 🧾 Features

- Converts file mode bits (from `std::fs::Metadata`) into symbolic `rwx` string format
- Supports regular files, directories, symlinks, and other file types
- No unnecessary dependencies — pure and minimal

## 🔧 Example

### display_permissions

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

### get_owner_and_group

```rust
use std::fs;
use unix_perms::get_owner_and_group;

fn main() {
    let path = ".";

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().into_string().unwrap_or("???".to_string());
            let (owner, group) = get_owner_and_group(entry);
            println!("{:<20} owner: {:<10} group: {}", file_name, owner, group);
        }
    }
}
```

### get_name

```rust
use unix_perms::get_name;
fn main() {
    let id = 0;
    let name = get_name(id);
    println!("{}", name); //Output: root
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
