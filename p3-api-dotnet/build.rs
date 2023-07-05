fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/structs/ship.rs")
        .csharp_dll_name("p3_api_dotnet")
        .generate_csharp_file("NativeMethods.g.cs")
        .unwrap();
}
