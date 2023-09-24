fn main() {
    let rustc_version = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to get rustc version");

    let cargo_version = std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to get cargo version");

    let rustc_version_str = String::from_utf8(rustc_version.stdout).expect("Invalid UTF-8");
    let cargo_version_str = String::from_utf8(cargo_version.stdout).expect("Invalid UTF-8");

    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version_str);
    println!("cargo:rustc-env=CARGO_VERSION={}", cargo_version_str);
    let script = format!(
    "pub fn rustc_version() -> &'static str {{ return \"{}\"; }} pub fn cargo_version() -> &'static str {{ return \"{}\"; }}",
    rustc_version_str.trim(),
    cargo_version_str.trim()
);
    std::fs::write("src/libs/version.rs", script).expect("Failed to write version.rs");
}
