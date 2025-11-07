use std::path::PathBuf;

fn main() {
    // Get the Swiss Ephemeris source directory
    let sweph_dir = PathBuf::from(r"E:\Claude Projects\EZ Astro Calculator\swisseph-master");
    
    // Compile Swiss Ephemeris C files
    cc::Build::new()
        .file(sweph_dir.join("sweph.c"))
        .file(sweph_dir.join("swephlib.c"))
        .file(sweph_dir.join("swecl.c"))
        .file(sweph_dir.join("swehouse.c"))
        .file(sweph_dir.join("swedate.c"))
        .file(sweph_dir.join("swejpl.c"))
        .file(sweph_dir.join("swemmoon.c"))
        .file(sweph_dir.join("swemplan.c"))
        .include(&sweph_dir)
        .warnings(false)
        .compile("swe");
    
    // Tell cargo to look for libraries in the sweph directory
    println!("cargo:rustc-link-search=native={}", sweph_dir.display());
    
    // Set up ephemeris path as an environment variable
    let ephe_path = sweph_dir.join("ephe");
    println!("cargo:rustc-env=SWEPH_PATH={}", ephe_path.display());
    
    // Note: No tauri_build::build() needed for Dioxus!
}
