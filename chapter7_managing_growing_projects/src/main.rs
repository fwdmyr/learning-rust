// Convention:
// - src/main.rs is the crate root of a binary crate with the same name as the package (found in the
//   Cargo.toml file).
// - src/lib.rs is the crate root of the binary crate.
// - src/bin is the directory where additional binary crates live.

// A crate's functionality lives in its own namespace that coincides with the crate's name.

fn main() {
    println!("Hello, world!");
}
