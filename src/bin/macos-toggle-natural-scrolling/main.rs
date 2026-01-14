use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main()
{
    // Embed the binary at compile time
    const OBJC_BINARY: &[u8] =
        include_bytes!(env!("TOGGLE_NATURAL_SCROLLING_BIN"));

    // Extract to a temporary location
    let temp_dir = std::env::temp_dir();
    let bin_path = temp_dir.join("ToggleNaturalScrolling");

    if !bin_path.exists()
    {
        // Write the binary
        println!("Writing binary to path: {}", bin_path.display());
        let mut file = fs::File::create(&bin_path)
            .expect("Failed to create temporary binary");
        file.write_all(OBJC_BINARY).expect("Failed to write binary data");

        println!("Making executable at path: {}", bin_path.display());
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin_path, perms).unwrap();
    }

    println!("Running ToggleNaturalScrolling at path: {}", bin_path.display());
    std::process::Command::new(&bin_path)
        .status()
        .expect(":: Could not run ToggleNaturalScrolling");
}
