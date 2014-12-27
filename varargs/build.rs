use std::io::Command;
use std::os;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["src/clib.c", "-c", "-fPIC", "-o"])
        .arg(format!("{}/clib.o", out_dir))
        .status().unwrap();

    Command::new("ar")
        .args(&["crus", "libclib.a", "clib.o"])
        .cwd(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-flags=-L {} -l clib:static", out_dir);
}
