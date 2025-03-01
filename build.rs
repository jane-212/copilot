fn main() {
    #[cfg(debug_assertions)]
    println!("cargo:rustc-env=VERSION=0.0.0");
}
