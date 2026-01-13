fn main() {
    println!("cargo:rustc-link-arg-bins=/ENTRY:main");

    println!("cargo:rustc-link-arg-bins=/NOLOGO");
    println!("cargo:rustc-link-arg-bins=/FILEALIGN:512");
    println!("cargo:rustc-link-arg-bins=/ALIGN:16");
    println!("cargo:rustc-link-arg-bins=/OPT:REF");
    println!("cargo:rustc-link-arg-bins=/MERGE:.rdata=.");
    println!("cargo:rustc-link-arg-bins=/MERGE:.text=.");
    println!("cargo:rustc-link-arg-bins=/MERGE:.pdata=.");
    println!("cargo:rustc-link-arg-bins=/SECTION:.,ER");

    println!("cargo:rustc-link-arg-bins=/NODEFAULTLIB");
    println!("cargo:rustc-link-lib=user32");
}
