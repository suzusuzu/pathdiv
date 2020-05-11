# pathdiv

![Rust](https://github.com/suzusuzu/pathdiv/workflows/Rust/badge.svg) 
[![Document](https://docs.rs/pathdiv/badge.svg)](https://docs.rs/pathdiv) 
[![Crates](https://img.shields.io/crates/v/pathdiv.svg)](https://crates.io/crates/pathdiv)

This library provides an API similar to pathlib of python.
"/" can be used to join the path.
PathDiv is a wrapper for [PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html).

# Example

```rust
use pathdiv::PathDiv;

let mut path = PathDiv::from("/etc");
path = path / "init.d";
path /= "ssh";
println!("{}", path); // "/etc/init.d/ssh"
```
