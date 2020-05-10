# pathdiv

This library provides an API similar to pathlib of python.
"/" can be used to join the path.
PathDiv is a wrapper for PathBuf.

# Example

```rust
use pathdiv::PathDiv;

let mut path = PathDiv::from("/etc");
path = path / "init.d";
path /= "ssh";
println!("{}", path); // "/etc/init.d/init.d/ssh"
```
