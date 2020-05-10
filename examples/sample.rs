extern crate pathdiv;

use pathdiv::PathDiv;

fn main() {
    let mut path = PathDiv::from("/etc");
    path = path / "init.d";
    path /= "ssh";
    println!("{}", path); // "/etc/init.d/init.d/ssh"
}
