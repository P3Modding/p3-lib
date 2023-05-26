fn main() {
    println!("cargo:rustc-link-search=C:\\Users\\Benni\\repositories\\p3-lib\\p3-aim-sys\\blobs");
    println!("cargo:rustc-link-lib=AIM");
    println!("cargo:rustc-link-lib=ijl11");
}
