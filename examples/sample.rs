extern crate pathdiv;
use pathdiv::PathDiv;

fn main() {
    let mut path = PathDiv::from("/etc");
    println!("{}", path);
    println!("{}", &path / "init.d");
    path /= "init.d";
    println!("{}", path);
    println!("{}", path / "init.d" / "ssh");
}