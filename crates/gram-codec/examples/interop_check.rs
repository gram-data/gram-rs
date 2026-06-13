//! Interop check: read gram from stdin, write JSON interchange format to stdout.
//!
//! Used by tests/interop/run.sh to compare output across Rust, TypeScript, and Python.
//!
//! Run with:
//!   cargo run -q -p relateby-gram --example interop_check < tests/interop/quatrain.gram

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim_start().starts_with("//"))
        .for_each(|l| {
            input.push_str(&l);
            input.push('\n');
        });

    match gram_codec::json::gram_parse_to_json(&input) {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}
