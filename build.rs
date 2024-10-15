use std::process::Command;

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=src/icon.png");
    let output = Command::new("npx")
        .args(&["nwlink", "png-nwi", "src/icon.png", "target/icon.nwi"])
        .output();
    if let Err(e) = output {
        println!("Failed to create the icon : {}", e);
        println!("Writing zeros instead.");
        // TODO : write 4250 bytes of zeros to target/icon.nwi
    };
}
