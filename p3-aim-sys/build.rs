fn main() {
    println!("cargo:rustc-link-lib=AIM");
    println!("cargo:rustc-link-lib=ijl11");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
    std::fs::copy("blobs/AIM.dll", out_dir.join("AIM.dll")).unwrap();
    std::fs::copy("blobs/AIM.lib", out_dir.join("AIM.lib")).unwrap();
    std::fs::copy("blobs/ijl11.dll", out_dir.join("ijl11.dll")).unwrap();
    std::fs::copy("blobs/ijl11.lib", out_dir.join("ijl11.lib")).unwrap();
}
