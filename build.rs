use std::process::Command;

fn main() {
    let status = Command::new("glib-compile-schemas")
        .arg("data")
        .status()
        .expect("Failed to compile schemas");

    if !status.success() {
        panic!("Failed to compile schemas {:?}", status);
    }
}
