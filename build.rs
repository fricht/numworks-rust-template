use nwi_compress::compress_icon;

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=src/icon.png");
    if let Err(e) = compress_icon("src/icon.png", "target/icon.nwi") {
        panic!("{}", e);
    };
}
