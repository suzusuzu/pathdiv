extern crate pathdiv;

use pathdiv::PathDiv;

#[cfg(target_os = "linux")]
#[test]
fn div_linux() {
    let mut path = PathDiv::from("/etc");
    path = path / "init.d";
    assert_eq!("/etc/init.d", path.to_str().unwrap())
}

#[cfg(target_os = "linux")]
#[test]
fn div_assign_linux() {
    let mut path = PathDiv::from("/etc");
    path /= "init.d";
    assert_eq!("/etc/init.d", path.to_str().unwrap())
}


#[cfg(target_os = "windows")]
#[test]
fn div_windows() {
    let mut path = PathDiv::from(r"C:\");
    path = path / "Windows";
    assert_eq!(r"C:\Windows", path.to_str().unwrap())
}

#[cfg(target_os = "windows")]
#[test]
fn div_assign_windows() {
    let mut path = PathDiv::from(r"C:\");
    path /= "Windows";
    assert_eq!(r"C:\Windows", path.to_str().unwrap())
}
