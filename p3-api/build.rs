fn main() {
    #[cfg(feature = "bindings-dotnet")]
    csbindgen();
}

#[cfg(feature = "bindings-dotnet")]
fn csbindgen() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/data/enums.rs")
        .input_extern_file("src/export/mod.rs")
        .input_extern_file("src/dotnet/mod.rs")
        .csharp_class_accessibility("public")
        .csharp_namespace("P3Api")
        .csharp_dll_name("p3_api")
        .generate_csharp_file("NativeMethods.g.cs")
        .unwrap();
}
