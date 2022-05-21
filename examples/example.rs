// it even resolves macros and prelude items fine
#![no_implicit_prelude]

// magic_import does not try to resolve which crates to import
// these would normally be included by prelude
extern crate magic_import;
extern crate std;

magic_import::magic!();

fn main() {
    let _ = BTreeMap::<File, PathBuf>::new();
    if let Ok(i) = i32::from_str("123") {
        print!("{i}");
    } else {
        unreachable!();
    }
    std::io::stdout().write_all(b"!\n").unwrap();
}
