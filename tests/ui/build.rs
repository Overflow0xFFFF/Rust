#![warn(clippy::print_stdout)]

fn main() {
    // Fix #6041
    //
    // The `print_stdout` shouldn't be linted in `build.rs`
    // as these methods are used for the build script.
    println!("Hello");
    print!("Hello");
}
