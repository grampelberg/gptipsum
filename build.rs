//! Build script for kuberift.

static KEY: &str = "API_KEY";

fn main() {
    if let Some(key) = std::env::var_os(KEY) {
        println!("cargo:rustc-env={}={}", KEY, key.to_string_lossy());
    }
}
