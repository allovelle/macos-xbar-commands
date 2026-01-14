fn main()
{
    let bin = env!("TOGGLE_NATURAL_SCROLLING_BIN"); // build.rs
    println!("Running ToggleNaturalScrolling at path: {}", bin);
    std::process::Command::new(bin)
        .status()
        .expect(":: Could not run ToggleNaturalScrolling");
}
