fn main() {
    println!("Hello Rust!!");
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../../solutions/hello_rust/Cargo.toml";
    #[test]
    fn test_hello() {
        let out = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .output()
            .expect("Failed to execute command");

        println!("{:?}", String::from_utf8(out.stderr));
        assert!(out.status.success());
        assert_eq!(String::from_utf8(out.stdout).unwrap(), "Hello, Rust!!\n");
    }
}
