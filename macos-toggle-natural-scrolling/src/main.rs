fn main()
{
    let bin = env!("TOGGLE_NATURAL_SCROLLING_BIN"); // build.rs
    std::process::Command::new(bin)
        .status()
        .expect("failed to run ToggleNaturalScrolling");
}
